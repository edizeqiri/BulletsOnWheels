mod weapon;

use bevy::log::LogPlugin;
use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
        filter: "info,wgpu_core=error,wgpu_hal=error,rustydefense=debug".into(),
        level: bevy::log::Level::DEBUG,
        custom_layer: |_| None,
    }))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}








