use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use godot_bevy::prelude::*;

use crate::character::{CharacterCore, Health, MovementSpeed};
use crate::gamestate::InGameState;
use crate::world::level_manager::{CurrentLevel, LevelId};
// use crate::gamestate::EnemyResource;
// use crate::weapon::Weapons;
// use crate::world::map::map::Level;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<EnemySpawnedMessage>()
        .add_message::<CreateEnemyMessage>()
        .add_systems(
            Update,
            spawn_enemy_system
                .run_if(in_state(LevelId::Level1))
                .run_if(in_state(InGameState::RUNNING))
        );
}

#[derive(Component, Default)]
pub struct Enemy;

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

#[derive(Message)]
pub struct EnemySpawnedMessage {
    pub entity: Entity
}

#[derive(Message)]
pub struct CreateEnemyMessage {
    pub position: Vec2
}

fn spawn_enemy_system(
    mut enemy_spawn_mesage: MessageReader<CreateEnemyMessage>,
    // mut godot: GodotAccess,
    mut commands: Commands,
    assets: Option<Res<EnemyAssets>>,
    current_level: Res<CurrentLevel>
) {
    let Some(assets) = assets else {
        return;
    };

    for message in enemy_spawn_mesage.read() {
        let Some(level) = current_level.entity else {
            return;
        };

        let enemy = commands
            .spawn((
                GodotScene::from_handle(assets.enemy_scene.clone()),
                Transform::from_xyz(message.position.x, message.position.y, 0.)
            ))
            .id();

        commands.entity(level).add_child(enemy);
    }
}
