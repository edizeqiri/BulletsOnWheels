use bevy::prelude::*;

use crate::weapon::Damage;
use crate::weapon_impl::Speed;

#[derive(Default, Bundle)]
pub struct ProjectileBundle {
    projectile: Projectile,
    damage: Damage,
    velocity: Velocity,
    speed: Speed
}

#[derive(Component, Default)]
pub struct Projectile;

#[derive(Component, Default, Clone)]
pub struct Velocity(pub Vec2);

pub fn create_projectile(damage: Damage, speed: Speed, direction: Vec2) -> ProjectileBundle {
    ProjectileBundle {
        projectile: Projectile,
        damage: damage,
        // TODO(bug): it should be only normalize
        velocity: Velocity(direction.normalize_or_zero()),
        speed: speed
    }
}
