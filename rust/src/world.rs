use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use godot::classes::{Button, NinePatchRect};
use godot::classes::class_macros::private::virtuals::ZipReader::Vector2;
use godot_bevy::prelude::*;

use crate::gamestate::{AppState, InGameState};
use crate::level_manager::LevelId::{self, Level1, MainMenu};
use crate::level_manager::{self, CurrentLevel, LoadLevelMessage};
use crate::score::NameEnteredEvent;
use crate::{level1, menu};

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(AppState::RUNNING).load_collection::<WorldAssets>())
        .add_observer(reset_game)
        .add_plugins(level1::plugin)
        .add_plugins(menu::plugin)
        .add_plugins(level_manager::plugin)
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
            Update,
            connect_restart_buttons.run_if(
                in_state(InGameState::PAUSED)
                    .or_else(in_state(InGameState::DEFEAT).and_then(in_state(Level1)))
            )
        )
        .add_observer(despawn_ask_for_player_name_system)
        .add_observer(on_mouse_enter_button_animation)
        .add_observer(on_mouse_exit_button_animation);
}

fn init_world(mut commands: Commands) {
    commands.trigger(LoadLevelMessage { level_id: MainMenu });
}

#[derive(Component, Default, GodotNode)]
#[godot_node(base(Button), class_name(RRestartButton))]
pub struct RestartButton {
    pub is_connected: bool
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
            |_args, restart_handle, _ent| {
                info!("Mouse hovered over button.");
                Some(ButtonEnteredEvent {
                    button_handle: restart_handle,
                })
            }
        );

        button_exited_signal.connect(
            *restart_handle,
            CollisionObject2DSignals::MOUSE_EXITED,
            None,
            |_args, restart_handle, _ent| {
                info!("Mouse leaving button.");
                Some(ButtonExitedEvent {
                    button_handle: restart_handle,
                })
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

#[derive(Event, Debug, Clone)]
pub struct PauseGameEvent;

#[derive(Event, Debug, Clone)]
pub struct ExitPauseGameEvent;

#[derive(Event, Debug, Clone)]
pub struct RestartGameEvent;

#[derive(Event, Debug, Clone)]
pub struct ButtonEnteredEvent {
    pub button_handle: GodotNodeHandle
}

#[derive(Event, Debug, Clone)]
pub struct ButtonExitedEvent {
    pub button_handle: GodotNodeHandle
}

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

fn on_mouse_enter_button_animation(
    trigger: On<ButtonEnteredEvent>,
    mut godot: GodotAccess
) {
    let Some(button) = godot.try_get::<Button>(trigger.button_handle) else {
        info!("No button found for this handle.");
        return;
    };

    // sprite of the button
    let Some(mut nine_patch_rect) = button.try_get_node_as::<NinePatchRect>("RestartSprite") else {
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

fn on_mouse_exit_button_animation(
    trigger: On<ButtonExitedEvent>
    , mut godot: GodotAccess
) {
    let Some(button) = godot.try_get::<Button>(trigger.button_handle) else {
        info!("No button found for this handle.");
        return;
    };

    // sprite of the button
    let Some(mut nine_patch_rect) = button.try_get_node_as::<NinePatchRect>("RestartSprite") else {
        info!("No child found of type NinePatchRect");
        return;
    };

    // RESET ANIMATION
    nine_patch_rect.set_modulate(godot::prelude::Color::from_rgb(1.0, 1.0, 1.0));
    nine_patch_rect.set_scale(Vector2::new(1., 1.));
}