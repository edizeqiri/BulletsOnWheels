mod weapon;
mod character;
mod projectile;
use crate::weapon::Shootable;
use crate::weapon::Weapon;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use crate::character::player::Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(logger()))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, init_player)
        .add_systems(FixedUpdate ,shoot_every_second)
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

fn shoot_every_second(
    mut commands: Commands,
    query: Query<(&Weapon,With<Player>)>
) {
    for weapon in &query {
        weapon.shoot()
    }
}

