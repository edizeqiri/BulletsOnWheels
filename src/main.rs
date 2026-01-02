mod character;
mod gamepad;
mod physics;
mod projectile;
mod setup;
mod terrain;
mod weapon;

use bevy::log::LogPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(logger())
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(physics::plugin)
        .add_plugins(gamepad::plugin)
        .add_plugins(setup::plugin)
        .add_plugins(weapon::plugin)
        .add_plugins(character::plugin)
        .add_plugins(terrain::plugin)
        .run();
}

fn logger() -> LogPlugin {
    LogPlugin {
        filter: "info,wgpu_core=error,wgpu_hal=error,BulletOnWheels=debug".into(),
        level: bevy::log::Level::DEBUG,
        custom_layer: |_| None,
        fmt_layer: |_| None,
    }
}
