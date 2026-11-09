use avian2d::debug_render::PhysicsDebugPlugin;
use bevy::asset::UnapprovedPathMode;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use dev_tools::physics_config::PhysicsConfigUI;
use crate::game::MD;

mod character_controller;
mod game;
mod combat;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    unapproved_path_mode: UnapprovedPathMode::Allow,
                    ..default()
                }),
        )
        .add_plugins(LdtkPlugin)
        .add_plugins(MD)
        // Dev Tools
        .add_plugins(PhysicsDebugPlugin)
        .add_plugins(PhysicsConfigUI)
        .run();
}