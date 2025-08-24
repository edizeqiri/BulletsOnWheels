mod character;
mod gamepad;
mod physics;
mod projectile;
mod setup;
mod weapon;

use bevy::log::LogPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(logger()))
        .add_plugins(physics::plugin)
        .add_plugins(gamepad::plugin)
        .add_plugins(setup::plugin)
        .add_plugins(weapon::plugin)
        .run();
}

fn logger() -> LogPlugin {
    LogPlugin {
        filter: "info,wgpu_core=error,wgpu_hal=error,rustydefense=debug".into(),
        level: bevy::log::Level::DEBUG,
        custom_layer: |_| None,
    }
}
