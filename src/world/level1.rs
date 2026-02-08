use bevy::app::{App, FixedUpdate};
use bevy::log::debug;
use bevy::prelude::*;
use bevy::state::state::OnEnter;
use bevy::time::{Fixed, Time};
use rand::prelude::*;

use crate::character::enemy::create_enemy_bundle;
use crate::character::player::Player;
use crate::gamestate::start::ENEMY_DEFAULTS;
use crate::gamestate::{EnemyResource, GameState};
use crate::weapon::Weapons;
use crate::world::map::Map;
use crate::world::simple_map::SimpleMap;
use crate::world::walls::create_wall_bundle;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_seconds(3.))
        .insert_resource(ENEMY_DEFAULTS)
        .add_systems(OnEnter(GameState::RUNNING), generate_level1_map_system)
        .add_systems(
            FixedUpdate,
            spawn_enemies_after_time.run_if(in_state(GameState::RUNNING))
        )
        .add_systems(Update, update_camera);
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
    let mut map = SimpleMap::default();
    let mut start = Vec2::new(0., 0.);
    map.add_path(start, 10);
    map.get_paths().iter().for_each(|path| {
        debug!("path: {:?}", path);
        path.points.iter().for_each(|vertice| {
            commad.spawn(create_wall_bundle(Transform::from_xyz(
                vertice.x, vertice.y, 0.
            )));
        });
    });
}
fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .translation
        .smooth_nudge(&direction, 2.0, time.delta_secs());
}
