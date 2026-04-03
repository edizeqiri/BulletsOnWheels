use bevy::prelude::*;

use crate::character::Health;
use crate::character::enemy::Enemy;
use crate::gamestate::GameState;
use crate::world::map::{Level, Map};
use crate::world::walls::create_wall_bundle;

mod infinity_map;
pub(crate) mod level1;
pub mod map;
mod simple_map;
mod walls;
pub(super) fn plugin(app: &mut App) {
    app.init_state::<LevelState>()
        .add_systems(OnEnter(GameState::START), clean_up)
        .add_systems(OnEnter(GameState::START), generate_level0)
        .add_systems(OnEnter(LevelState::ZERO), generate_level0)
        .add_systems(OnExit(LevelState::ZERO), clean_up)
        .add_plugins(level1::plugin);
}

fn clean_up(mut command: Commands, level: Single<Entity, With<Level>>) {
    command.entity(level.entity()).despawn()
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum LevelState {
    #[default]
    ZERO,
    ONE,
    TWO
}

// TODO: Use Map Trait
fn generate_level0(mut command: Commands) {
    let (w, h) = (1280, 640);

    command
        .spawn((
            Name::new("Level 0"),
            Level(LevelState::ZERO),
            Visibility::default(),
            Transform::default()
        ))
        .with_children(|command| {
            for x in (0..w).step_by(20) {
                let x_pos = x as f32 - w as f32 / 2.0;
                command.spawn(create_wall_bundle(Transform::from_xyz(
                    x_pos,
                    h as f32 / 2.0,
                    0.0
                ))); // Top
                command.spawn(create_wall_bundle(Transform::from_xyz(
                    x_pos,
                    -h as f32 / 2.0,
                    0.0
                ))); // Bottom
            }

            // Vertical walls (left & right)
            for y in (0..h).step_by(20) {
                let y_pos = y as f32 - h as f32 / 2.0;
                command.spawn(create_wall_bundle(Transform::from_xyz(
                    -w as f32 / 2.0,
                    y_pos,
                    0.0
                ))); // Left
                command.spawn(create_wall_bundle(Transform::from_xyz(
                    w as f32 / 2.0,
                    y_pos,
                    0.0
                ))); // Right
            }
        });
}
