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
use crate::world::level::level1;
use crate::world::level_manager::{CurrentLevel, LoadLevelMessage};
mod level;
pub(crate) mod level_manager;
pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(AppState::RUNNING).load_collection::<WorldAssets>())
        .add_plugins(level1::plugin)
        .add_plugins(level_manager::LevelManagerPlugin)
        //.add_systems(Update, (spawn_death_scene_on_player_death, track_death_scene))
        .add_plugins(GodotSignalsPlugin::<NameEnteredEvent>::default())
        .add_systems(
            OnEnter(InGameState::DEFEAT),
            (spawn_ask_for_player_name_system, connect_enter_name_system),
        )
        .add_observer(name_submitted);
}

#[derive(Event, Clone)]
struct NameEnteredEvent {
    name: String,
}

#[derive(AssetCollection, Resource)]
pub struct WorldAssets {
    #[asset(path = "scenes/defeat/player_death_restart.tscn")]
    pub death_scene_restart: Handle<GodotResource>,

    #[asset(path = "scenes/defeat/player_death_highscore.tscn")]
    pub player_death_highscore_scene: Handle<GodotResource>,
}

fn spawn_ask_for_player_name_system(mut commands: Commands, assets: Res<WorldAssets>) {
    commands.spawn_empty().insert(GodotScene::from_handle(
        assets.player_death_highscore_scene.clone(),
    ));
}

fn connect_enter_name_system(
    text_field_object: Query<&GodotNodeHandle, With<LineEditMarker>>,
    entered_name_signal: GodotSignals<NameEnteredEvent>,
) {
    let Ok(handler) = text_field_object.single() else {
        info!("handler of line edit not found");
        return;
    };

    entered_name_signal.connect(
        *handler,
        LineEditSignals::TEXT_SUBMITTED,
        None,
        |args, _node_handle, _ent| {
            let Some(name) = args.get(0)?.try_to::<String>().ok() else {
                error!("Name could not be found or parsed");
                return None;
            };

            Some(NameEnteredEvent { name })
        },
    );
}

// todo(sascha): not triggered anymore. weiiiird
fn name_submitted(trigger: On<NameEnteredEvent>) {
    let entered_name = &trigger.event().name;
    info!("entered name {}", entered_name)
}

#[derive(Component)]
struct DeathTimer(Timer);

fn spawn_death_scene_on_player_death(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    assets: Option<Res<WorldAssets>>,
    mut player_death_message: MessageReader<CharacterDeathMessage>,
    player_query: Query<(), With<Player>>,
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
                    GodotScene::from_handle(assets.death_scene_restart.clone()),
                    DeathTimer(Timer::from_seconds(3., TimerMode::Once)),
                ))
                .id();

            commands.entity(level).add_child(scene);
        }
    }
}

fn track_death_scene(
    mut commands: Commands,
    time_query: Query<(&mut DeathTimer, Entity)>,
    time: Res<Time>,
) {
    for (mut times, entity) in time_query {
        if times.0.is_finished() {
            commands.entity(entity).despawn();
            commands.trigger(LoadLevelMessage {
                level_id: level_manager::LevelId::MainMenu,
            });
        } else {
            times.0.tick(time.delta());
        }
    }
}
