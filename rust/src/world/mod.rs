use std::thread::sleep;
use std::time::Duration;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use godot::register::info;
use godot_bevy::prelude::*;

use crate::character::player::Player;
use crate::gamestate::{AppState, CharacterDeathMessage, InGameState};
use crate::ui::score::SpawnLeaderBoardEvent;
use crate::world::level::level1;
use crate::world::level_manager::LevelId::{self, Level1, MainMenu};
use crate::world::level_manager::{CurrentLevel, LoadLevelMessage};
mod level;
pub(crate) mod level_manager;
mod menu;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(AppState::RUNNING).load_collection::<WorldAssets>())
        .add_observer(reset_game)
        .add_plugins(level1::plugin)
        .add_plugins(menu::plugin)
        .add_plugins(level_manager::LevelManagerPlugin)
        .add_systems(Startup, init_world)
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
        .add_plugins(GodotSignalsPlugin::<NameEnteredEvent>::default())
        .add_systems(
            OnEnter(InGameState::DEFEAT),
            spawn_ask_for_player_name_system.run_if(in_state(Level1))
        )
        .add_systems(Update, connect_enter_name_system)
        .add_observer(despawn_ask_for_player_name_system);
}

fn init_world(mut commands: Commands) {
    commands.trigger(LoadLevelMessage { level_id: MainMenu });
}

#[derive(Event, Clone, Default)]
pub struct NameEnteredEvent {
    pub name: String
}

#[derive(AssetCollection, Resource)]
pub struct WorldAssets {
    #[asset(path = "scenes/defeat/player_death_restart.tscn")]
    pub death_scene_restart: Handle<GodotResource>,

    #[asset(path = "scenes/defeat/player_death_highscore.tscn")]
    pub player_death_highscore_scene: Handle<GodotResource>
}

#[derive(Component)]
pub struct DeathHighscoreScene;

fn spawn_ask_for_player_name_system(
    mut commands: Commands,
    assets: Res<WorldAssets>,
) {
    commands
        .spawn_empty()
        .insert(GodotScene::from_handle(assets.player_death_highscore_scene.clone()))
        .insert(DeathHighscoreScene);
}

fn despawn_ask_for_player_name_system(
    _trigger: On<SpawnLeaderBoardEvent>,
    death_high_score_query: Query<Entity, With<DeathHighscoreScene>>,
    mut commands: Commands
) {
    let Ok(deaht_high_score_scene) = death_high_score_query.single() else {
        error!("Could not despawn death highscore scene.");
        return;
    };
    commands.entity(deaht_high_score_scene).despawn();
}

fn connect_enter_name_system(
    mut connected: Local<bool>,
    enter_name_field: Query<&GodotNodeHandle, With<LineEditMarker>>,
    entered_name_signal: GodotSignals<NameEnteredEvent>
) {
    if *connected {
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
    *connected = true;
}


#[derive(Component)]
struct DeathTimer(Timer);

#[derive(Event, Debug, Clone)]
pub struct PauseGameEvent;

#[derive(Event, Debug, Clone)]
pub struct ExitPauseGameEvent;

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

#[derive(Debug, Event)]
pub struct ResetSceneEvent(pub Entity);

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
            commands
                .entity(entity)
                .queue_silenced(|mut e: EntityWorldMut| {
                    e.despawn();
                });
            commands.trigger(RestartGameEvent);
        } else {
            times.0.tick(time.delta());
        }
    }
}
