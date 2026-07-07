use bevy::{app::App, ecs::{component::Component, system::Query}, log::info};
use godot_bevy::{interop::{BaseButtonSignals, GodotNodeHandle}, plugins::signals::GodotSignals};
use godot_bevy_macros::GodotNode;
use bevy::prelude::*;
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState}};
use godot_bevy::prelude::*;

use crate::{gamestate::{AppState, InGameState}, level_manager::{CurrentLevel, LevelId::{self, Level1}}, score::NameEnteredEvent, world::RestartGameEvent};

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(AppState::RUNNING).load_collection::<WorldAssets>())
        .add_plugins(GodotSignalsPlugin::<NameEnteredEvent>::default())
        .add_systems(
            Update,
            connect_restart_buttons.run_if(
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
    ;
}

#[derive(Component, Default, GodotNode)]
#[godot_node(base(Button), class_name(RRestartButton))]
pub struct RestartButton {
    pub is_connected: bool
}

fn connect_restart_buttons(
    mut restart_buttons: Query<(&GodotNodeHandle, &mut RestartButton)>,
    signal: GodotSignals<RestartGameEvent>
) {
    for (restart_handle, mut restart_button) in &mut restart_buttons {
        if restart_button.is_connected {
            continue;
        }

        signal.connect(
            *restart_handle,
            BaseButtonSignals::PRESSED,
            None,
            |_args, _node_handle, _ent| {
                info!("Restart button pressed");
                Some(RestartGameEvent)
            }
        );

        restart_button.is_connected = true;
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
