use bevy::app::App;
use bevy::prelude::{Commands, Name, OnEnter, OnExit, Transform, Visibility};

use crate::world::map::Level;
use crate::world::walls::create_wall_bundle;
use crate::world::{LevelState, clean_up};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(LevelState::ZERO), generate_level0)
        .add_systems(OnExit(LevelState::ZERO), clean_up);
}

// TODO: Use Map Trait
pub fn generate_level0(mut command: Commands) {
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
