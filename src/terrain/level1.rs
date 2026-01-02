use crate::character::enemy::create_enemy_bundle;
use crate::weapon::Weapons;
use bevy::app::{App, FixedUpdate};
use bevy::prelude::{Commands, Transform};
use bevy::time::{Fixed, Time};
use rand::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_seconds(3.))
        .add_systems(FixedUpdate, spawn_enemies_after_time);
}

fn spawn_enemies_after_time(mut command: Commands) {
    let mut rng = rand::rng();

    command.spawn(create_enemy_bundle(
        Transform::from_xyz(
            rng.random_range(-100.0..100.0),
            rng.random_range(-100.0..100.0),
            0.,
        ),
        Weapons::default(),
        10,
        "Enemy1",
    ));
}
