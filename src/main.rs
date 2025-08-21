mod character;
mod projectile;
mod weapon;

use crate::character::player::Player;
use crate::weapon::Shootable;
use crate::weapon::Weapon;
use bevy::input::keyboard::Key::Play;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use bevy::ecs::bundle::DynamicBundle;
use crate::projectile::{move_projectile, Projectile};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(logger()))
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, init_player)
        .add_systems(FixedUpdate, shoot_every_second)
        .add_systems(Update, projectile_system)
        .run();
}

fn logger() -> LogPlugin {
    LogPlugin {
        filter: "info,wgpu_core=error,wgpu_hal=error,rustydefense=debug".into(),
        level: bevy::log::Level::DEBUG,
        custom_layer: |_| None,
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn init_player(mut commands: Commands) {
    commands.spawn(character::get_random_player());
}

fn shoot_every_second(mut commands: Commands, query: Query<&Weapon, With<Player>>,time: Res<Time<Fixed>>) {
    for weapon in &query {
        commands.spawn((weapon.shoot(), Player));
    }
}

fn projectile_system(
    mut commands: Commands,
    query: Query<(&Projectile, &mut Transform)>
) {
    for (projectile, transform) in query {
        move_projectile(projectile, transform);
    }
}

