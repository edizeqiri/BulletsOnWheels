use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use core_engine::gamestate::{AppState, ExitGameEvent, InGameState};
use core_engine::world::{ExitPauseGameEvent, PauseGameEvent, RestartGameEvent};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::level_manager::CurrentLevel;

pub(crate) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(AppState::RUNNING).load_collection::<MenuAssets>())
        .init_resource::<MenuHandles>()
        .add_plugins(GodotSignalsPlugin::<RestartGameEvent>::default())
        .add_plugins(GodotSignalsPlugin::<ExitGameEvent>::default())
        .add_observer(pause_game_on_event.run_if(in_state(InGameState::RUNNING)))
        .add_observer(exit_pause.run_if(in_state(InGameState::PAUSED)))
        .add_systems(
            Update,
            (
                init_menu_assets.run_if(menu_not_initialized),
                connect_menu_buttons.run_if(menu_not_connected)
            )
                .run_if(in_state(InGameState::PAUSED))
        );
}

#[derive(AssetCollection, Resource)]
pub struct MenuAssets {
    #[asset(path = "scenes/menu/menu.tscn")]
    pub menu_scene: Handle<GodotResource>
}

#[derive(Resource, Default)]
pub struct MenuHandles {
    pub restart_button: Option<GodotNodeHandle>,
    pub exit_button: Option<GodotNodeHandle>,
    pub initialized: bool,
    pub signals_connected: bool
}

fn menu_not_initialized(menu_handles: Res<MenuHandles>) -> bool {
    !menu_handles.initialized
}

fn menu_not_connected(menu_handles: Res<MenuHandles>) -> bool {
    !menu_handles.signals_connected
}

#[derive(NodeTreeView)]
pub struct MenuUi {
    #[node("/root/Menu/Border/RestartButton")]
    pub restart_button: GodotNodeHandle,

    #[node("/root/Menu/Border/ExitButton")]
    pub exit_button: GodotNodeHandle
}

#[derive(Debug, Component)]
struct PauseMenu;

fn exit_pause(
    _event: On<ExitPauseGameEvent>,
    mut commands: Commands,
    current_state: Res<State<InGameState>>,
    menu_query: Query<Entity, With<PauseMenu>>,
    menu_handles: Option<ResMut<MenuHandles>>
) {
    if *current_state.get() == InGameState::PAUSED {
        let Ok(entity) = menu_query.single() else {
            return;
        };
        let Some(menu) = menu_handles else {
            return;
        };
        commands.entity(entity).queue_silenced(|e: EntityWorldMut| {
            e.despawn();
        });
        commands.set_state(InGameState::RUNNING);
        reset_menu(menu);
        info!("exit pause menu");
    }
}

fn reset_menu(mut menu: ResMut<MenuHandles>) {
    menu.initialized = false;
    menu.signals_connected = false;
    menu.restart_button = None;
    menu.exit_button = None;
}

fn pause_game_on_event(
    _event: On<PauseGameEvent>,
    assets: Option<Res<MenuAssets>>,
    current_level: Res<CurrentLevel>,
    mut commands: Commands
) {
    let Some(assets) = assets else {
        info!("No Assets");
        return;
    };

    let Some(level) = current_level.entity else {
        info!("No level");
        return;
    };

    commands.set_state(InGameState::PAUSED);

    let menu = commands
        .spawn((
            GodotScene::from_handle(assets.menu_scene.clone()),
            PauseMenu
        ))
        .id();
    commands.entity(level).add_child(menu);
    info!("Enter pause menu");
}

fn init_menu_assets(mut menu_handles: ResMut<MenuHandles>, mut scene_tree: SceneTreeRef) {
    if let Some(root) = scene_tree.get().get_root() {
        match MenuUi::from_node(root) {
            Ok(menu_ui) => {
                info!("Menu is loaded");
                menu_handles.restart_button = Some(menu_ui.restart_button);
                menu_handles.exit_button = Some(menu_ui.exit_button);
                menu_handles.initialized = true;
            },
            Err(_) => {
                warn!("Menu not loaded yet");
            }
        }
    }
}

fn connect_menu_buttons(
    mut menu_handles: ResMut<MenuHandles>,
    _signals_restart: GodotSignals<RestartGameEvent>,
    signals_exit: GodotSignals<ExitGameEvent>
) {
    if menu_handles.signals_connected {
        return;
    };

    let (Some(_restart_handle), Some(exit_handle)) =
        (menu_handles.restart_button, menu_handles.exit_button)
    else {
        return;
    };

    menu_handles.signals_connected = true;
    info!("Buttons connected");
}
