use avian2d::PhysicsPlugins;
use avian2d::prelude::*;
use bevy::asset::UnapprovedPathMode;
use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::character_controller::{CharacterControllerBundle, CharacterControllerPlugin};
mod character_controller;

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
        .run();
}