use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use godot_bevy::prelude::*;

use crate::gamestate::{AppState, InGameState};
use crate::level1;
use crate::level_manager::LevelId::{self, Level1, MainMenu};
use crate::level_manager::{CurrentLevel, LoadLevelMessage};
use crate::menu;
use crate::score::SpawnLeaderBoardEvent;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(AppState::RUNNING).load_collection::<WorldAssets>())
        .add_observer(reset_game)
        .add_plugins(level1::plugin)
        .add_plugins(menu::plugin)
        .add_plugins(crate::level_manager::LevelManagerPlugin)
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
        .add_systems(Update, connect_enter_name_system.run_if(in_state(Level1)))
        .add_observer(despawn_ask_for_player_name_system);
}

fn init_world(mut commands: Commands) {
    commands.trigger(LoadLevelMessage { level_id: MainMenu });
}

#[derive(Component, Default, GodotNode)]
#[godot_node(base(Button), class_name(RRestartButton))]
pub struct RestartButton;

#[derive(Event, Clone, Default)]
pub struct NameEnteredEvent {
    pub name: String
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
pub struct DeathHighscoreScene;
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

fn spawn_ask_for_player_name_system(mut commands: Commands, mut assets: ResMut<WorldAssets>) {
    commands
        .spawn_empty()
        .insert(GodotScene::from_handle(
            assets.player_death_highscore_scene.clone()
        ))
        .insert(DeathHighscoreScene);
    assets.is_connected = false;
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
        level_id: crate::level_manager::LevelId::MainMenu
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
            commands.entity(entity).queue_silenced(|e: EntityWorldMut| {
                e.despawn();
            });
            commands.trigger(RestartGameEvent);
        } else {
            times.0.tick(time.delta());
        }
    }
}
