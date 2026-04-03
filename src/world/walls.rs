use bevy::color::palettes::css::PINK;
use bevy::prelude::*;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::{ActiveEvents, Collider};

use crate::character::square_sprite;

pub(super) fn plugin(app: &mut App) {
    app;
}

#[derive(Component)]
#[require(RigidBody, Collider, ActiveEvents, Sprite, Transform)]
pub struct Wall;

#[derive(Bundle)]
pub struct WallBundle {
    wall: Wall,
    body: RigidBody,
    collider: Collider,
    active_events: ActiveEvents,
    transform: Transform,
    sprite: Sprite
}

pub fn create_wall_bundle(transform: Transform) -> WallBundle {
    WallBundle {
        wall: Wall,
        body: RigidBody::Fixed,
        collider: Collider::cuboid(10.0, 10.0),
        active_events: Default::default(),
        transform,
        sprite: square_sprite(Color::Srgba(PINK))
    }
}