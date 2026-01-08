use bevy::app::{App, FixedUpdate};
use bevy::log::debug;
use bevy::prelude::{in_state, Commands, IntoScheduleConfigs, Name, Res, Transform, Vec2};
use bevy::time::{Fixed, Time};
use rand::prelude::*;

use crate::character::enemy::create_enemy_bundle;
use crate::gamestate::start::ENEMY_DEFAULTS;
use crate::gamestate::{EnemyResource, GameState};
use crate::weapon::Weapons;
use crate::world::infinity_map::InfiniteMap;
use crate::world::map::Map;
use crate::world::simple_map::SimpleMap;

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

fn generate_level1_map_system() {
    let mut map = InfiniteMap::default();
    let mut map = SimpleMap::default();
    let mut start = Vec2::new(0., 0.);
    map.add_path(start, 3);
    // for x in (1..100) {
    //     start = map.add_path(start, 3);
    // }
}
