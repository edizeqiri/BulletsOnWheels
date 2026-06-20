use bevy::prelude::*;
use godot_bevy::prelude::*;

use crate::weapon::Damage;
use crate::weapon::weapon::Speed;

#[derive(Default, Bundle)]
pub struct ProjectileBundle {
    projectile: Projectile,
    damage: Damage,
    velocity: Velocity
}

#[derive(Component, Default)]
pub struct Projectile;

#[derive(Component, Default, Clone)]
pub struct Velocity(pub Vec2);

pub fn create_projectile(damage: Damage, speed: Speed, direction: Vec2) -> ProjectileBundle {
    ProjectileBundle {
        projectile: Projectile,
        damage: damage,
        velocity: Velocity(direction.normalize() * speed.0)
    }
}
