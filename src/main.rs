mod character;
mod gamepad;
mod gamestate;
mod physics;
mod projectile;
mod setup;
mod weapon;
mod world;
mod ui;

use crate::gamestate::GameState;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_lunex::UiLunexPlugins;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(logger())
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<GameState>()
        .add_plugins(UiLunexPlugins)
        .add_plugins(physics::plugin)
        .add_plugins(gamepad::plugin)
        .add_plugins(setup::plugin)
        .add_plugins(weapon::plugin)
        .add_plugins(character::plugin)
        .add_plugins(world::plugin)
        .add_plugins(gamestate::plugin)
        .add_plugins(ui::plugin)
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
