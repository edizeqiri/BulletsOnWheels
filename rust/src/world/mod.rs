use std::thread::sleep;
use std::time::Duration;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use godot_bevy::prelude::*;

use crate::character::player::Player;
use crate::gamestate::{AppState, CharacterDeathMessage, InGameState};
use crate::world::level::level1;
use crate::world::level_manager::{CurrentLevel, LoadLevelMessage};
mod level;
pub(crate) mod level_manager;
mod menu;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(AppState::RUNNING).load_collection::<WorldAssets>())
        .add_observer(reset_game)
        .add_plugins(level1::plugin)
        .add_plugins(level_manager::LevelManagerPlugin)
        .add_plugins(menu::plugin)
        .add_systems(
            Update,
            (spawn_death_scene_on_player_death, track_death_scene)
        )
        .insert_resource(Time::<Fixed>::from_seconds(1.5))
        .add_systems(FixedUpdate, log_system);
}

fn log_system(appstate: Res<State<AppState>>, ingamesteate: Res<State<InGameState>>) {
    info!("Current appstate: {:?}", appstate.get());
    info!("Current game: {:?}", ingamesteate.get());
}

#[derive(AssetCollection, Resource)]
pub struct WorldAssets {
    #[asset(path = "scenes/main_menu/player_death.tscn")]
    pub death_scene: Handle<GodotResource>
}

#[derive(Component)]
struct DeathTimer(Timer);

#[derive(Event, Debug, Clone)]
pub struct PauseGameEvent;

#[derive(Event, Debug, Clone)]
pub struct RestartGameEvent;

#[derive(Event, Debug, Clone)]
pub struct ExitGameEvent;

fn reset_game(_: On<RestartGameEvent>, mut commands: Commands) {
    commands.trigger(LoadLevelMessage {
        level_id: level_manager::LevelId::MainMenu
    });
    commands.set_state(InGameState::RUNNING);
}

fn spawn_death_scene_on_player_death(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    assets: Option<Res<WorldAssets>>,
    mut player_death_message: MessageReader<CharacterDeathMessage>,
    player_query: Query<(), With<Player>>
) {
    for message in player_death_message.read() {
        if player_query.get(message.target).is_ok() {
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
                    GodotScene::from_handle(assets.death_scene.clone()),
                    DeathTimer(Timer::from_seconds(3., TimerMode::Once))
                ))
                .id();

            commands.entity(level).add_child(scene);
        }
    }
}

fn track_death_scene(
    mut commands: Commands,
    time_query: Query<(&mut DeathTimer, Entity)>,
    time: Res<Time>
) {
    for (mut times, entity) in time_query {
        if times.0.is_finished() {
            commands.entity(entity).despawn();
            commands.trigger(LoadLevelMessage {
                level_id: level_manager::LevelId::MainMenu
            });
        } else {
            times.0.tick(time.delta());
        }
    }
}
