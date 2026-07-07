use bevy::prelude::*;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use core_engine::gamestate::AppState;
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::godot_adapter::{EnemyAssets, ProjectileAssets};
use crate::level_manager::LevelId;
mod gamepad;
mod godot_adapter;
mod input;
mod keyboard;
mod level1;
mod level_manager;
mod main_menu;
mod menu;
mod mouse;

#[bevy_app]
fn build_app(app: &mut App) {
    // GodotDefaultPlugins provides all standard godot-bevy functionality
    // For minimal setup, use individual plugins instead:
    // app.add_plugins(GodotTransformSyncPlugin)
    //     .add_plugins(GodotAudioPlugin)
    //     .add_plugins(BevyInputBridgePlugin);
    app.add_plugins(GodotDefaultPlugins)
        .add_plugins(godot_adapter::plugin)
        .add_plugins(core_engine::plugin)
        .init_state::<LevelId>()
        .add_loading_state(
            LoadingState::new(AppState::RUNNING)
                .load_collection::<ProjectileAssets>()
                .load_collection::<EnemyAssets>()
        );
}
