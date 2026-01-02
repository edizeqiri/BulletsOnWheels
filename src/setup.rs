use crate::character::enemy::create_enemy_bundle;
use crate::character::player::create_player_bundle;
use crate::weapon::bow::Bow;
use crate::weapon::gun::Gun;
use crate::weapon::{Weapon, Weapons};
use bevy::app::{App, Startup};
use bevy::prelude::{Camera2d, Commands, Transform};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera)
        .add_systems(Startup, init);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn init(mut commands: Commands) {
    let weapons = Weapons(vec![
        Weapon::Bow(Bow::new(1, 1000., 0.5)),
        Weapon::Gun(Gun::new(1, 250., 5.)),
    ]);

    commands.spawn(create_player_bundle(
        Transform::from_xyz(100.0, 0.0, 0.0),
        weapons.clone(),
        1000,
        "Player",
    ));

    commands.spawn(create_enemy_bundle(
        Transform::from_xyz(-100.0, 0.0, 0.0),
        weapons.clone(),
        1000,
        "Enemy1",
    ));

    commands.spawn(create_enemy_bundle(
        Transform::from_xyz(200.0, 0.0, 0.0),
        weapons,
        1000,
        "Enemy2",
    ));
}
