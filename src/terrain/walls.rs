use bevy::prelude::*;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::{ActiveEvents, Collider};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_perimeter_walls);
}

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Destructible;

#[derive(Bundle)]
struct WallBundle {
    body: RigidBody,
    collider: Collider,
    active_events: ActiveEvents,
    transform: Transform,
}

fn create_wall_bundle(transform: Transform) -> WallBundle {
    WallBundle {
        body: RigidBody::Fixed,
        collider: Collider::cuboid(10.0, 10.0),
        active_events: Default::default(),
        transform,
    }
}

pub fn spawn_perimeter_walls(mut commands: Commands) {
    let (w, h) = (640.0, 320.0);

    // Horizontal walls (top & bottom)
    for x in (0..640).step_by(20) {
        let x_pos = x as f32 - w / 2.0;
        commands.spawn((
            Destructible,
            Wall,
            create_wall_bundle(Transform::from_xyz(x_pos, h / 2.0, 0.0)),
        )); // Top
        commands.spawn((
            Destructible,
            Wall,
            create_wall_bundle(Transform::from_xyz(x_pos, -h / 2.0, 0.0)),
        )); // Bottom
    }

    // Vertical walls (left & right)
    for y in (0..320).step_by(20) {
        let y_pos = y as f32 - h / 2.0;
        commands.spawn((
            Destructible,
            Wall,
            create_wall_bundle(Transform::from_xyz(-w / 2.0, y_pos, 0.0)),
        )); // Left
        commands.spawn((
            Destructible,
            Wall,
            create_wall_bundle(Transform::from_xyz(w / 2.0, y_pos, 0.0)),
        )); // Right
    }
}
