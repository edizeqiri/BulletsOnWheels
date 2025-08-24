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
}

pub fn create_projectile(damage: u32, speed: f32, direction: Vec2) -> ProjectileBundle {
    ProjectileBundle {
        projectile: Projectile { damage },
        body: RigidBody::KinematicVelocityBased,
        velocity: Velocity::linear(speed * direction.normalize()),
        sensor: Sensor,
        collider: Collider::ball(10.0),
        active_events: ActiveEvents::COLLISION_EVENTS,
    }
}
