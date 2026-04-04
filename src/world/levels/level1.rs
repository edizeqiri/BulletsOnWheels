use bevy::app::{App, FixedUpdate};
use bevy::log::debug;
use bevy::prelude::*;
use bevy::state::state::OnEnter;
use bevy::time::{Fixed, Time};
use rand::prelude::*;

use crate::character::enemy::{EnemyType, create_enemy_bundle, CreateEnemyMessage};
use crate::gamestate::start::ENEMY_DEFAULTS;
use crate::gamestate::{EnemyResource, GameState};
use crate::weapon::Weapons;
use crate::world::map::{Level, Map};
use crate::world::simple_map::SimpleMap;
use crate::world::walls::create_wall_bundle;
use crate::world::{LevelState, clean_up};

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_seconds(3.))
        .insert_resource(ENEMY_DEFAULTS)
        .add_systems(OnEnter(LevelState::ONE), generate_level1_map_system)
        .add_systems(
            FixedUpdate,
            spawn_enemies_after_time
                .run_if(in_state(GameState::RUNNING))
                .run_if(in_state(LevelState::ONE))
        )
        .add_systems(OnExit(LevelState::ONE), clean_up);
}

fn spawn_enemies_after_time(
    mut enemy_writer: MessageWriter<CreateEnemyMessage>
) {
    enemy_writer.write(CreateEnemyMessage);
}

pub fn generate_level1_map_system(mut command: Commands) {
    let mut map = SimpleMap::default();
    let mut start = Vec2::new(100., 100.);
    command
        .spawn((
            Name::new("Level 1"),
            Level(LevelState::ONE),
            Visibility::default(),
            Transform::default()
        ))
        .with_children(|cmd| {
            map.add_path(start, 10.);
            let paths = map.get_paths().clone(); // clone needed, because of map ownership
            cmd.spawn((
                Name::new("SimpleMap"),
                map,
                Transform::default(),
                Visibility::default()
            ))
            .with_children(|cmd| {
                paths.iter().for_each(|path| {
                    path.points.iter().for_each(|vertice| {
                        cmd.spawn(create_wall_bundle(Transform::from_xyz(
                            vertice.x, vertice.y, 0.
                        )));
                    });
                });
            });
        });
}
