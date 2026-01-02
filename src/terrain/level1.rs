use crate::character::enemy::create_enemy_bundle;
use crate::weapon::Weapons;
use bevy::app::{App, FixedUpdate};
use bevy::log::debug;
use bevy::prelude::{Commands, Name, Res, Transform};
use bevy::time::{Fixed, Time};
use rand::prelude::*;
use crate::gamestate::EnemyProperties;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_seconds(3.))
        .insert_resource(EnemyProperties::default())
        .add_systems(FixedUpdate, spawn_enemies_after_time);
}

fn spawn_enemies_after_time(
    mut command: Commands,
    enemy_properties: Res<EnemyProperties>
) {
    let mut rng = rand::rng();

    debug!("spawn enemy with max health: {}", enemy_properties.max_health);
    command.spawn(create_enemy_bundle(
        Transform::from_xyz(
            rng.random_range(-100.0..100.0),
            rng.random_range(-100.0..100.0),
            0.,
        ),
        Weapons::default(),
        enemy_properties.max_health,
        enemy_properties.name.clone(),
    ));
}
