use crate::character::square_sprite;
use bevy::color::palettes::css::PINK;
use bevy::prelude::*;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::{ActiveEvents, Collider};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_perimeter_walls);
}

#[derive(Component)]
#[require(RigidBody, Collider, ActiveEvents, Sprite, Transform)]
struct Wall;

#[derive(Bundle)]
struct WallBundle {
    wall: Wall,
    body: RigidBody,
    collider: Collider,
    active_events: ActiveEvents,
    transform: Transform,
    sprite: Sprite,
}

fn create_wall_bundle(transform: Transform) -> WallBundle {
    WallBundle {
        wall: Wall,
        body: RigidBody::Fixed,
        collider: Collider::cuboid(10.0, 10.0),
        active_events: Default::default(),
        transform,
        sprite: square_sprite(Color::Srgba(PINK)),
    }
}

pub fn spawn_perimeter_walls(mut commands: Commands) {
    let (w, h) = (2 * 640, 2 * 320);
    // Horizontal walls (top & bottom)
    for x in (0..w).step_by(20) {
        let x_pos = x as f32 - w as f32 / 2.0;
        commands.spawn((create_wall_bundle(Transform::from_xyz(
            x_pos,
            h as f32 / 2.0,
            0.0,
        )),)); // Top
        commands.spawn((create_wall_bundle(Transform::from_xyz(
            x_pos,
            -h as f32 / 2.0,
            0.0,
        )),)); // Bottom
    }

    // Vertical walls (left & right)
    for y in (0..h).step_by(20) {
        let y_pos = y as f32 - h as f32 / 2.0;
        commands.spawn((create_wall_bundle(Transform::from_xyz(
            -w as f32 / 2.0,
            y_pos,
            0.0,
        )),)); // Left
        commands.spawn((create_wall_bundle(Transform::from_xyz(
            w as f32 / 2.0,
            y_pos,
            0.0,
        )),)); // Right
    }
}
