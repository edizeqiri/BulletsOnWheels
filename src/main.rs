#![feature(trivial_bounds)]

mod character;
mod gamestate;
pub mod input;
mod physics;
mod projectile;
mod setup;
mod ui;
mod weapon;
mod world;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_lunex::UiLunexPlugins;

use crate::gamestate::GameState;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(logger())
                .set(ImagePlugin::default_nearest())
        )
        .init_state::<GameState>()
        .add_plugins(UiLunexPlugins)
        .add_plugins(EguiPlugin::default())
        //.add_plugins(WorldInspectorPlugin::new())
        .add_plugins(physics::plugin)
        .add_plugins(input::plugin)
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
        fmt_layer: |_| None
    }
}
