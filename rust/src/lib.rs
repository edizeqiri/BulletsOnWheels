use bevy::{prelude::*, state::app::StatesPlugin};
use bevy_asset_loader::{asset_collection::AssetCollectionApp, loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState}};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::gamestate::{GameState};
use crate::weapon::weapon::ProjectileAssets;
mod character;

mod gamestate;
mod input;
mod level_manager;
mod main_menu;
mod weapon;

#[bevy_app]
fn build_app(app: &mut App) {
    // GodotDefaultPlugins provides all standard godot-bevy functionality
    // For minimal setup, use individual plugins instead:
    // app.add_plugins(GodotTransformSyncPlugin)
    //     .add_plugins(GodotAudioPlugin)
    //     .add_plugins(BevyInputBridgePlugin);
    app.add_plugins(GodotDefaultPlugins)
        .add_plugins(weapon::plugin)
        .add_plugins(character::plugin)
        .add_plugins(input::plugin)
        .add_plugins(level_manager::LevelManagerPlugin)
        .add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .add_loading_state(LoadingState::new(GameState::START)
            .load_collection::<ProjectileAssets>());
}
