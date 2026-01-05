use bevy::app::{App, FixedUpdate};
use bevy::log::debug;
use bevy::prelude::{Commands, IntoScheduleConfigs, Name, Res, Transform, Vec2, in_state};
use bevy::time::{Fixed, Time};
use rand::prelude::*;

use crate::character::enemy::create_enemy_bundle;
use crate::gamestate::start::ENEMY_DEFAULTS;
use crate::gamestate::{EnemyResource, GameState};
use crate::weapon::Weapons;
use crate::world::infinity_map::generate_map;
use crate::world::map::Map;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_seconds(3.))
        .insert_resource(ENEMY_DEFAULTS) // will be something else that depends on state = running
        .add_systems(
            FixedUpdate,
            spawn_enemies_after_time.run_if(in_state(GameState::RUNNING))
        );
}

fn spawn_enemies_after_time(mut command: Commands, enemy_properties: Res<EnemyResource>) {
    let mut rng = rand::rng();

    debug!(
        "spawn enemy with max health: {}",
        enemy_properties.max_health
    );
    command.spawn(create_enemy_bundle(
        Transform::from_xyz(
            rng.random_range(-100.0..100.0),
            rng.random_range(-100.0..100.0),
            0.
        ),
        Weapons::default(),
        enemy_properties.max_health,
        Name::from("Enemy".to_string() + rng.next_u32().to_string().as_str())
    ));
}

fn map_system() {
    let mut map = generate_map();
    map.add_path(Vec2::new(0., 0.));
}
