use std::collections::HashMap;

use bevy::app::App;
use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::character::player::Player;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(setup_direction_map())
        .add_systems(
            Update,
            debug_linvel.run_if(resource_changed::<ButtonInput<KeyCode>>)
        )
        .add_systems(Update, keyboard_aim_system);
}

fn debug_linvel(mut player_query: Query<&Velocity, With<Player>>) {
    if let Ok(velocity) = player_query.single_mut() {
        debug!("Current velocity is {}", velocity.linvel)
    }
}

fn setup_direction_map() -> DirectionMap {
    let mut map: HashMap<DirectionType, (Vec<KeyCode>, Vec2)> = HashMap::new();
    map.insert(
        DirectionType::Up,
        (vec![KeyCode::ArrowUp, KeyCode::KeyW], vec2(0., 1.))
    );
    map.insert(
        DirectionType::Down,
        (vec![KeyCode::ArrowDown, KeyCode::KeyS], vec2(0., -1.))
    );
    map.insert(
        DirectionType::Left,
        (vec![KeyCode::ArrowLeft, KeyCode::KeyA], vec2(-1., 0.))
    );
    map.insert(
        DirectionType::Right,
        (vec![KeyCode::ArrowRight, KeyCode::KeyD], vec2(1., 0.))
    );
    DirectionMap { bindings: map }
}

// key dir type, val vector keycodes, vector direction
#[derive(Eq, PartialEq, Hash)]
enum DirectionType {
    Up,
    Down,
    Left,
    Right
}

#[derive(Resource)]
struct DirectionMap {
    bindings: HashMap<DirectionType, (Vec<KeyCode>, Vec2)>
}

//
fn keyboard_aim_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    directions_map: Res<DirectionMap>,
    mut player_query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = player_query.single_mut() {
        // TODO: Hardcoded speed should be a resource
        let speed = 200.;
        let mut direction_vector: Vec2 = vec2(0., 0.);

        directions_map
            .bindings
            .iter()
            .for_each(|(_, (keycodes, dir))| {
                if keyboard_input.any_pressed(keycodes.iter().copied()) {
                    direction_vector += dir;
                }
            });
        velocity.linvel = direction_vector.normalize_or_zero() * speed;
    }
}
