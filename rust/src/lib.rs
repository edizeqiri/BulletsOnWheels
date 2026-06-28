use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::enemy::EnemyAssets;
use crate::gamestate::{AppState, InGameState};
use crate::level_manager::LevelId;
use crate::weapon_impl::ProjectileAssets;

mod character;
mod debug;
mod enemy;
mod enemy_ai;
mod gamepad;
mod gamestate;
mod input;
mod keyboard;
mod level1;
mod level_manager;
mod main_menu;
mod menu;
mod mouse;
mod player;
mod projectile;
mod score;
mod weapon;
mod weapon_impl;
mod world;

#[bevy_app]
fn build_app(app: &mut App) {
    // GodotDefaultPlugins provides all standard godot-bevy functionality
    // For minimal setup, use individual plugins instead:
    // app.add_plugins(GodotTransformSyncPlugin)
    //     .add_plugins(GodotAudioPlugin)
    //     .add_plugins(BevyInputBridgePlugin);
    app.add_plugins(GodotDefaultPlugins)
        .add_plugins(debug::plugin)
        .add_plugins(world::plugin)
        .add_plugins(weapon::plugin)
        .add_plugins(character::plugin)
        .add_plugins(input::plugin)
        .add_plugins(main_menu::plugin)
        .add_plugins(score::plugin)
        .add_plugins(gamestate::plugin)
        .add_plugins(StatesPlugin)
        .init_state::<AppState>()
        .init_state::<InGameState>()
        .init_state::<LevelId>()
        .add_loading_state(
            LoadingState::new(AppState::RUNNING)
                .load_collection::<ProjectileAssets>()
                .load_collection::<EnemyAssets>()
        );
}
