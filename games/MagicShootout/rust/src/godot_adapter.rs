use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use godot::classes::{AnimatedSprite2D, CharacterBody2D, Input};
use godot::global::Key;
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::character::{
    CharacterCore, CharacterHitMessage, CollisionStartedDomain, Health, MovementDirection,
    MovementSpeed
};
use crate::enemy::{Enemy, EnemySpawnedMessage};
#[cfg(not(target_arch = "wasm32"))]
use crate::gamestate::ExitGameEvent;
use crate::gamestate::InGameState;
use crate::level_manager::LevelId::MainMenu;
use crate::level_manager::LoadLevelMessage;
use crate::weapon::Damage;
use crate::weapon_impl::{FireRate, ProjectileShot, Speed, WeaponKindComponent};
use crate::world::RestartGameEvent;
use crate::{input, level_manager, level1, menu};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GodotTransformConfig::two_way())
        .add_plugins(input::plugin)
        .add_observer(collision_adapter)
        .add_systems(
            PhysicsUpdate,
            apply_character_movement.run_if(in_state(InGameState::RUNNING))
        )
        .add_systems(Update, update_healthbar_animation_system)
        .add_systems(Update, log_scene_tree_on_keypress)
        .add_observer(exit_game)
        .add_observer(reset_game)
        .add_plugins(level1::plugin)
        .add_plugins(menu::plugin)
        .add_plugins(level_manager::plugin)
        .add_systems(Startup, init_world);
}
fn collision_adapter(collision: On<CollisionStarted>, mut commands: Commands) {
    let event = collision.event();
    commands.trigger(CollisionStartedDomain {
        entity1: event.entity1,
        entity2: event.entity2
    });
}

#[derive(Bundle, GodotNode)]
#[godot_node(base(CharacterBody2D), class_name(REnemy2D))]
pub struct EnemyBundle {
    enemy: Enemy,

    #[export_fields(max(export_type(f32), default(10.)))]
    health: Health,

    #[export_fields(value(export_type(f32), default(100.)))]
    speed: MovementSpeed,

    core: CharacterCore
}

#[derive(AssetCollection, Resource)]
pub struct EnemyAssets {
    #[asset(path = "scenes/characters/enemy.tscn")]
    pub enemy_scene: Handle<GodotResource>
}

/// Movement Sink, godot style
/// move_and_slide is needed so that we do not have teleports by just changing
/// translation in transform
fn apply_character_movement(
    query: Query<(&GodotNodeHandle, &MovementDirection, &MovementSpeed)>,
    mut godot: GodotAccess
) {
    for (handle, movement, speed) in &query {
        let Some(mut body) = godot.try_get::<CharacterBody2D>(*handle) else {
            continue;
        };
        let velocity = Vector2::new(movement.vec.x, movement.vec.y) * speed.0;
        body.set_velocity(velocity);
        body.move_and_slide();
    }
}

fn update_healthbar_animation_system(
    mut damage_message: MessageReader<CharacterHitMessage>,
    characters: Query<&GodotNodeHandle, With<Health>>,
    mut godot: GodotAccess
) {
    for damage in damage_message.read() {
        if let Ok(handle) = characters.get(damage.target) {
            let Some(character_body) = godot.try_get::<CharacterBody2D>(*handle) else {
                return;
            };
            let mut sprite = character_body.get_node_as::<AnimatedSprite2D>("Healthbar");

            if damage.health >= 0. && damage.health <= 4. {
                let number: u32 = damage.health as u32;
                sprite.play_ex().name(&number.to_string()).done();
            }
        }
    }
}

fn log_scene_tree_on_keypress(
    mut scene_tree: SceneTreeRef,
    mut godot: GodotAccess,
    mut was_pressed: Local<bool>
) {
    let gd_input = godot.singleton::<Input>();
    let pressed = gd_input.is_key_pressed(Key::H);

    if pressed && !*was_pressed {
        if let Some(root) = scene_tree.get().get_root() {
            info!("=== Scene Tree ===");
            log_node(&root.upcast::<Node>());
            info!("==================");
        }
    }

    *was_pressed = pressed;
}

fn log_node(node: &Gd<Node>) {
    info!("{} [{}]", node.get_path(), node.get_class());
    for child in node.get_children().iter_shared() {
        log_node(&child);
    }
}

fn spawn_enemy_scene(
    mut enemy_spawn_mesage: MessageReader<EnemySpawnedMessage>,
    mut commands: Commands,
    assets: Option<Res<EnemyAssets>>
) {
    let Some(assets) = assets else {
        return;
    };

    for message in enemy_spawn_mesage.read() {
        commands
            .entity(message.entity)
            .insert(GodotScene::from_handle(assets.enemy_scene.clone()));
    }
}

#[derive(GodotNode, Component, Default)]
#[godot_node(base(Label), class_name(RExitGameLabel))]
pub struct ExitGameLabel;

#[cfg(not(target_arch = "wasm32"))]
fn exit_game(
    _trigger: On<ExitGameEvent>,
    mut exit: MessageWriter<AppExit>,
    mut scene_tree: SceneTreeRef
) {
    info!("Exit game.");

    // bevy exit
    exit.write(AppExit::Success);

    // godot exit
    scene_tree.get().quit();
}

#[cfg(target_arch = "wasm32")]
fn exit_game(
    _trigger: On<ExitGameEvent>,
    handles: Query<&GodotNodeHandle, With<ExitGameLabel>>,
    mut godot: GodotAccess
) {
    let Ok(handle) = handles.single() else {
        return;
    };
    let Some(mut label) = godot.try_get::<Label>(*handle) else {
        return;
    };
    label.set_visible(true);
}

#[derive(Component, GodotNode, Default, Clone)]
#[godot_node(base(Node2D), class_name(RWeapon2D))]
pub struct Weapon {
    #[export_fields(value(export_type(f32), default(1.)))]
    damage: Damage,

    // TODO(bug): this export is somehow not working
    pub speed: Speed,

    #[export_fields(value(export_type(f32), default(0.)))]
    fire_rate: FireRate,

    #[export_fields(value(export_type(WeaponKind), default(WeaponKind::GUN)))]
    weapon_kind: WeaponKindComponent
}

#[derive(GodotConvert, Var, Export, Default, Clone)]
#[godot(via = GString)] // provides enum as string
pub enum WeaponKind {
    #[default]
    GUN,
    BOW,
    STAFF
}

#[derive(AssetCollection, Resource)]
pub(crate) struct ProjectileAssets {
    #[asset(path = "scenes/characters/projectile.tscn")]
    pub projectile_scene: Handle<GodotResource>
}

fn on_shoot_adapter(
    mut commands: Commands,
    mut shoot_message: MessageReader<ProjectileShot>,
    assets: Option<Res<ProjectileAssets>>
) {
    // If the projectile assets are not yet loaded/inserted, consume any queued
    // shoot messages (to avoid a burst once assets arrive) and skip spawning
    // projectiles.
    let assets = match assets {
        Some(a) => a,
        None => {
            for _ in shoot_message.read() { /* drop events until assets are ready */ }
            return;
        }
    };
    for message in shoot_message.read() {
        commands
            .entity(message.projectile)
            .insert(GodotScene::from_handle(assets.projectile_scene.clone()));
    }
}

fn reset_game(_: On<RestartGameEvent>, mut commands: Commands) {
    commands.trigger(LoadLevelMessage {
        level_id: crate::level_manager::LevelId::MainMenu
    });
    commands.set_state(InGameState::RUNNING);
}

#[derive(Debug, Event)]
pub struct ResetSceneEvent(pub Entity);

fn init_world(mut commands: Commands) {
    commands.trigger(LoadLevelMessage { level_id: MainMenu });
}
