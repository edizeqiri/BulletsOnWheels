use bevy::prelude::*;
use godot_bevy::prelude::*;

use crate::{
    weapon::{Damage, weapon::Speed},
};

#[derive(Default, Bundle, GodotNode)]
#[godot_node(base(Area2D), class_name(RProjectile2D))]
pub struct ProjectileBundle {
    projectile: Projectile,
    #[export_fields(value(export_type(f32), default(1.)))]
    damage: Damage,
    velocity: Velocity,
}

#[derive(Component, Default)]
pub struct Projectile;

#[derive(Component, Default, Clone)]
pub struct Velocity(pub Vec2);

pub fn create_projectile(damage: Damage, speed: Speed, direction: Vec2) -> ProjectileBundle {
    ProjectileBundle {
        projectile: Projectile,
        damage: damage,
        velocity: Velocity(direction.normalize() * speed.0),
    }
}
