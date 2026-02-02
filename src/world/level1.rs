use bevy::app::{App, FixedUpdate};
use bevy::color::palettes::css::VIOLET;
use bevy::color::Color;
use bevy::log::debug;
use bevy::prelude::{in_state, Commands, IntoScheduleConfigs, Name, Res, Transform, Vec2};
use bevy::state::state::OnEnter;
use bevy::time::{Fixed, Time};
use rand::prelude::*;

use crate::character::enemy::create_enemy_bundle;
use crate::character::square_sprite;
use crate::gamestate::start::ENEMY_DEFAULTS;
use crate::gamestate::{EnemyResource, GameState};
use crate::weapon::Weapons;
use crate::world::infinity_map::InfiniteMap;
use crate::world::map::Map;
use crate::world::simple_map::SimpleMap;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_seconds(3.))
        .insert_resource(ENEMY_DEFAULTS)
        .add_systems(OnEnter(GameState::START), generate_level1_map_system); // will
                                                                             // be something
                                                                             // else
                                                                             // that
                                                                             // depends
                                                                             // on state = running
                                                                             // .add_systems(
                                                                             //     FixedUpdate,
                                                                             //     spawn_enemies_after_time.
                                                                             // run_if(in_state(GameState::RUNNING))
                                                                             // );
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

fn generate_level1_map_system(mut commad: Commands) {
    // let mut map = InfiniteMap::default();
    let mut map = SimpleMap::default();
    let mut start = Vec2::new(0., 0.);
    map.add_path(start, 30);
    map.get_paths().iter().for_each(|path| {
        debug!("path: {:?}", path);
        path.vertices.iter().for_each(|vertice| {
            commad.spawn((
                Transform::from_xyz(vertice.x, vertice.y, 0.),
                square_sprite(Color::Srgba(VIOLET))
            ));
        });
    });
    // for x in (1..3) {
    //     start = map.add_path(start, 3);
    // }
}
