use bevy::app::App;
use bevy::ecs::component::Component;
use bevy::ecs::system::Query;
use bevy::log::info;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use core_engine::character::{
    CharacterCore, CharacterHitMessage, CollisionStartedDomain, Health, MovementDirection,
    MovementSpeed
};
use core_engine::enemy::{Enemy, EnemySpawnedMessage};
#[cfg(not(target_arch = "wasm32"))]
use core_engine::gamestate::ExitGameEvent;
use core_engine::gamestate::{AppState, InGameState};
use core_engine::player::Player;
use core_engine::score::{
    DeathHighscoreScene, LiveScore, NameEnteredEvent, ScoreBoard, SpawnLeaderBoardEvent
};
use core_engine::weapon::Damage;
use core_engine::weapon_impl::{FireRate, ProjectileShot, Speed};
use core_engine::world::RestartGameEvent;
use godot::classes::{
    AnimatedSprite2D, Button, CharacterBody2D, Input, Label, NinePatchRect, RichTextLabel
};
use godot::global::Key;
use godot::prelude::*;
use godot_bevy::interop::{BaseButtonSignals, GodotNodeHandle};
use godot_bevy::plugins::signals::GodotSignals;
use godot_bevy::prelude::*;
use godot_bevy_macros::GodotNode;

use crate::level_manager::LevelId::{self, Level1, MainMenu};
use crate::level_manager::{CurrentLevel, LoadLevelMessage};
use crate::{input, level_manager, level1, main_menu, menu};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GodotTransformConfig::two_way())
        .add_plugins(input::plugin)
        .add_plugins(level1::plugin)
        .add_plugins(menu::plugin)
        .add_plugins(level_manager::plugin)
        .add_plugins(main_menu::plugin)
        .add_observer(collision_adapter)
        .add_systems(
            PhysicsUpdate,
            apply_character_movement.run_if(in_state(InGameState::RUNNING))
        )
        .add_systems(Update, update_healthbar_animation_system)
        .add_systems(Update, log_scene_tree_on_keypress)
        .add_observer(exit_game)
        .add_observer(reset_game)
        .add_systems(Startup, init_world)
        .add_loading_state(LoadingState::new(AppState::RUNNING).load_collection::<WorldAssets>())
        .add_loading_state(
            LoadingState::new(AppState::RUNNING).load_collection::<ScoreBoardAssets>()
        )
        .add_plugins(GodotSignalsPlugin::<NameEnteredEvent>::default())
        .add_systems(
            Update,
            connect_restart_buttons.run_if(
                in_state(InGameState::PAUSED)
                    .or_else(in_state(InGameState::DEFEAT).and_then(in_state(Level1)))
            )
        )
        .add_systems(
            Update,
            connect_exit_buttons.run_if(
                in_state(InGameState::PAUSED)
                    .or_else(in_state(InGameState::DEFEAT).and_then(in_state(Level1)))
            )
        )
        .add_systems(
            OnEnter(InGameState::DEFEAT),
            spawn_death_scene_on_player_death.run_if(in_state(LevelId::MainMenu))
        )
        .add_systems(
            Update,
            track_death_scene
                .run_if(in_state(InGameState::DEFEAT))
                .run_if(in_state(LevelId::MainMenu))
        )
        .add_systems(
            Update,
            update_score_board
                .run_if(in_state(InGameState::DEFEAT))
                .run_if(in_state(LevelId::Level1))
        )
        .add_systems(
            Update,
            update_score_label
                .run_if(in_state(LevelId::Level1))
                .run_if(in_state(InGameState::RUNNING))
        )
        .add_systems(
            OnEnter(InGameState::DEFEAT),
            spawn_ask_for_player_name_system.run_if(in_state(LevelId::Level1))
        )
        .add_systems(
            Update,
            connect_enter_name_system.run_if(in_state(LevelId::Level1))
        )
        .add_systems(Update, on_shoot_adapter)
        .add_observer(spawn_score_board)
        .add_observer(on_mouse_enter_button_animation)
        .add_observer(on_mouse_exit_button_animation);
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
    fire_rate: FireRate
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

#[derive(Bundle, GodotNode)]
#[godot_node(base(CharacterBody2D), class_name(RPlayer2D))]
pub struct PlayerBundle {
    player: Player,

    #[export_fields(
        current(export_type(f32), default(10.)),
        max(export_type(f32), default(10.))
    )]
    health: Health,

    #[export_fields(value(export_type(f32), default(200.)))]
    speed: MovementSpeed,

    core: CharacterCore
}

#[derive(Component, Default, GodotNode)]
#[godot_node(base(Button), class_name(RRestartButton))]
pub struct RestartButton {
    pub is_connected: bool
}

#[derive(Component, Default, GodotNode)]
#[godot_node(base(Button), class_name(RExitButton))]
pub struct ExitButton {
    pub is_connected: bool
}

#[derive(Event, Debug, Clone)]
pub struct ButtonEnteredEvent {
    pub button_handle: GodotNodeHandle
}

#[derive(Event, Debug, Clone)]
pub struct ButtonExitedEvent {
    pub button_handle: GodotNodeHandle
}

fn connect_restart_buttons(
    mut restart_buttons: Query<(&GodotNodeHandle, &mut RestartButton)>,
    restart_game_signal: GodotSignals<RestartGameEvent>,
    button_entered_signal: GodotSignals<ButtonEnteredEvent>,
    button_exited_signal: GodotSignals<ButtonExitedEvent>
) {
    for (restart_handle, mut restart_button) in &mut restart_buttons {
        if restart_button.is_connected {
            continue;
        }

        restart_game_signal.connect(
            *restart_handle,
            BaseButtonSignals::PRESSED,
            None,
            |_args, _node_handle, _ent| {
                info!("Restart button pressed");
                Some(RestartGameEvent)
            }
        );

        button_entered_signal.connect(
            *restart_handle,
            CollisionObject2DSignals::MOUSE_ENTERED,
            None,
            |_args, node_handle, _ent| {
                info!("Mouse hovered over button.");
                Some(ButtonEnteredEvent {
                    button_handle: node_handle
                })
            }
        );

        button_exited_signal.connect(
            *restart_handle,
            CollisionObject2DSignals::MOUSE_EXITED,
            None,
            |_args, node_handle, _ent| {
                info!("Mouse leaving button.");
                Some(ButtonExitedEvent {
                    button_handle: node_handle
                })
            }
        );

        restart_button.is_connected = true;
    }
}

fn connect_exit_buttons(
    mut exit_buttons: Query<(&GodotNodeHandle, &mut ExitButton)>,
    exit_game_signal: GodotSignals<ExitGameEvent>,
    button_entered_signal: GodotSignals<ButtonEnteredEvent>,
    button_exited_signal: GodotSignals<ButtonExitedEvent>
) {
    for (exit_handle, mut exit_button) in &mut exit_buttons {
        if exit_button.is_connected {
            continue;
        }

        exit_game_signal.connect(
            *exit_handle,
            BaseButtonSignals::PRESSED,
            None,
            |_args, _node_handle, _ent| {
                info!("Exit button pressed");
                Some(ExitGameEvent)
            }
        );

        button_entered_signal.connect(
            *exit_handle,
            CollisionObject2DSignals::MOUSE_ENTERED,
            None,
            |_args, restart_handle, _ent| {
                info!("Mouse hovered over button.");
                Some(ButtonEnteredEvent {
                    button_handle: restart_handle
                })
            }
        );

        button_exited_signal.connect(
            *exit_handle,
            CollisionObject2DSignals::MOUSE_EXITED,
            None,
            |_args, restart_handle, _ent| {
                info!("Mouse leaving button.");
                Some(ButtonExitedEvent {
                    button_handle: restart_handle
                })
            }
        );

        exit_button.is_connected = true;
    }
}

#[derive(AssetCollection, Resource)]
pub struct WorldAssets {
    #[asset(path = "scenes/defeat/player_death_restart.tscn")]
    pub death_scene_restart: Handle<GodotResource>,

    #[asset(path = "scenes/defeat/player_death_highscore.tscn")]
    pub player_death_highscore_scene: Handle<GodotResource>,
    pub is_connected: bool
}

#[derive(Component)]
struct DeathTimer(Timer);

fn spawn_death_scene_on_player_death(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    assets: Option<Res<WorldAssets>>
) {
    let Some(ref assets) = assets else {
        info!("player death asset not loaded yet");
        return;
    };

    let Some(level) = current_level.entity else {
        info!("No level id");
        return;
    };

    let scene = commands
        .spawn((
            GodotScene::from_handle(assets.death_scene_restart.clone()),
            DeathTimer(Timer::from_seconds(3., TimerMode::Once))
        ))
        .id();

    commands.entity(level).add_child(scene);
}

fn track_death_scene(
    mut commands: Commands,
    time_query: Query<(&mut DeathTimer, Entity)>,
    time: Res<Time>
) {
    for (mut times, entity) in time_query {
        if times.0.is_finished() {
            commands.entity(entity).queue_silenced(|e: EntityWorldMut| {
                e.despawn();
            });
            commands.trigger(RestartGameEvent);
        } else {
            times.0.tick(time.delta());
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct ScoreBoardAssets {
    #[asset(path = "scenes/defeat/score_board.tscn")]
    pub score_board_scene: Handle<GodotResource>,

    score_board_is_loaded: bool
}

fn spawn_score_board(
    _trigger: On<SpawnLeaderBoardEvent>,
    mut commands: Commands,
    mut assets: ResMut<ScoreBoardAssets>
) {
    commands
        .spawn_empty()
        .insert(GodotScene::from_handle(assets.score_board_scene.clone()))
        .insert(DespawnOnEnter(InGameState::RUNNING));
    assets.score_board_is_loaded = false;
}

fn update_score_board(
    score_board: Res<ScoreBoard>,
    mut scene_tree: SceneTreeRef,
    mut assets: ResMut<ScoreBoardAssets>
) {
    if assets.score_board_is_loaded {
        return;
    }

    let content = prepare_leader_board_content(score_board);

    assets.score_board_is_loaded = update_label(&mut scene_tree, "/root/ScoreBoard/Top5", content);
}

fn update_label(scene_tree: &mut SceneTreeRef, label_path: &str, content: String) -> bool {
    let Some(root) = scene_tree.get().get_root() else {
        return false;
    };

    let Some(mut label) = root.try_get_node_as::<RichTextLabel>(label_path) else {
        return false;
    };

    label.set_use_bbcode(true);
    label.set_text(&content);
    return true;
}

fn prepare_leader_board_content(score_board: Res<ScoreBoard>) -> String {
    let mut text = String::from("[table=3]");

    // Header
    text.push_str("[cell][left][b]Rank[/b][/left][/cell]");
    text.push_str("[cell][left][b]Name[/b][/left][/cell]");
    text.push_str("[cell][right][b]Score[/b][/right][/cell]");

    // Rows
    for (i, entry) in score_board.entries.iter().take(5).enumerate() {
        text.push_str(&format!(
            "[cell][left]{}.[/left][/cell]\
             [cell][left]{}[/left][/cell]\
             [cell][right]{}[/right][/cell]",
            i + 1,
            entry.name,
            entry.score
        ));
    }

    text.push_str("[/table]");
    text
}

fn connect_enter_name_system(
    enter_name_field: Query<&GodotNodeHandle, With<LineEditMarker>>,
    entered_name_signal: GodotSignals<NameEnteredEvent>,
    world_assets: Option<ResMut<WorldAssets>>
) {
    let Some(mut assets) = world_assets else {
        return;
    };

    if assets.is_connected {
        return;
    }
    let Ok(enter_name_handler) = enter_name_field.single() else {
        return;
    };

    entered_name_signal.connect(
        *enter_name_handler,
        LineEditSignals::TEXT_SUBMITTED,
        None,
        |args, _node_handle, _ent| {
            let Some(name) = args.get(0)?.try_to::<String>().ok() else {
                error!("Name could not be found or parsed");
                return None;
            };

            Some(NameEnteredEvent { name })
        }
    );
    info!("enter name signal connected");
    assets.is_connected = true;
}

fn spawn_ask_for_player_name_system(mut commands: Commands, mut assets: ResMut<WorldAssets>) {
    commands
        .spawn_empty()
        .insert(GodotScene::from_handle(
            assets.player_death_highscore_scene.clone()
        ))
        .insert(DeathHighscoreScene);
    assets.is_connected = false;
}

fn update_score_label(
    score_query: Query<&LiveScore, Changed<LiveScore>>,
    current_level: Res<CurrentLevel>,
    mut scene_tree: SceneTreeRef
) {
    let level_id = current_level.level_id;

    let Ok(score) = score_query.single() else {
        return;
    };

    let score_label_path = format!("{}/ScoreLabel", level_id.root_node_path());

    let Some(root) = scene_tree.get().get_root() else {
        warn!("no root");
        return;
    };

    let Some(mut score_label) = root.try_get_node_as::<Label>(&score_label_path) else {
        warn!("Could not find Label at {}", score_label_path);
        return;
    };

    score_label.set_text(&format!(
        "Score: {}\nHigh Score: {}",
        score.count, score.highscore,
    ));
}

fn on_mouse_enter_button_animation(trigger: On<ButtonEnteredEvent>, mut godot: GodotAccess) {
    let Some(button) = godot.try_get::<Button>(trigger.button_handle) else {
        info!("No button found for this handle.");
        return;
    };

    // sprite of the button
    let Some(mut nine_patch_rect) = button.try_get_node_as::<NinePatchRect>("ButtonSprite") else {
        info!("No child found of type NinePatchRect");
        return;
    };

    // ANIMATE BUTTON
    // make brighter
    nine_patch_rect.set_modulate(godot::prelude::Color::from_rgb(1.2, 1.2, 1.2));
    // increase size of button
    let size = nine_patch_rect.get_size();
    nine_patch_rect.set_pivot_offset(size / 2.0);
    nine_patch_rect.set_scale(Vector2::new(1.05, 1.05));
}

fn on_mouse_exit_button_animation(trigger: On<ButtonExitedEvent>, mut godot: GodotAccess) {
    let Some(button) = godot.try_get::<Button>(trigger.button_handle) else {
        info!("No button found for this handle.");
        return;
    };

    // sprite of the button
    let Some(mut nine_patch_rect) = button.try_get_node_as::<NinePatchRect>("ButtonSprite") else {
        info!("No child found of type NinePatchRect");
        return;
    };

    // RESET ANIMATION
    nine_patch_rect.set_modulate(godot::prelude::Color::from_rgb(1.0, 1.0, 1.0));
    nine_patch_rect.set_scale(Vector2::new(1., 1.));
}
