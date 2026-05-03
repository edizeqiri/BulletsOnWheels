mod character;
mod gamestate;
pub mod input;
mod physics;
mod projectile;
mod setup;
//mod ui;
mod weapon;
mod world;

use crate::gamestate::GameState;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

use godot::prelude::*;
use godot_bevy::prelude::*;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(GodotDefaultPlugins)
        .add_plugins(StatesPlugin)
        .add_plugins(physics::plugin)
        .add_plugins(input::plugin)
        .add_plugins(setup::plugin)
        .add_plugins(weapon::plugin)
        .add_plugins(character::plugin)
        .add_plugins(world::plugin)
        .add_plugins(gamestate::plugin);
    //.add_plugins(ui::plugin)
}

fn logger() -> LogPlugin {
    LogPlugin {
        filter: "info,wgpu_core=error,wgpu_hal=error,BulletOnWheels=debug".into(),
        level: bevy::log::Level::DEBUG,
        custom_layer: |_| None,
        fmt_layer: |_| None,
    }
}
