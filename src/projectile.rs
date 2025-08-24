use crate::character::{square_sprite, ENEMY_GROUP, PLAYER_GROUP};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub damage: u32,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    projectile: Projectile,
    velocity: Velocity,
    body: RigidBody,
    sensor: Sensor,
    collider: Collider,
    active_events: ActiveEvents,
    transform: Transform,
}

pub fn create_projectile(damage: u32, speed: f32, direction: Vec2) -> ProjectileBundle {
    ProjectileBundle {
        projectile: Projectile { damage },
        body: RigidBody::KinematicVelocityBased,
        velocity: Velocity::linear(speed * direction),
        sensor: Sensor,
        transform: Default::default(),
        collider: Collider::ball(10.0),
        active_events: ActiveEvents::COLLISION_EVENTS,
    }
}


