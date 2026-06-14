use bevy::prelude::*;
use godot_bevy::prelude::*;

use crate::weapon::{Damage, weapon::Speed};

#[derive(Component, Default)]
pub struct Projectile {
    pub damage: Damage,
}

#[derive(Default, Bundle, GodotNode)]
#[godot_node(base(Area2D), class_name(RProjectile2D))]
pub struct ProjectileBundle {
    projectile: Projectile,
}

pub fn create_projectile(damage: Damage, speed: Speed, direction: Vec2) -> ProjectileBundle {
    ProjectileBundle {
        projectile: Projectile { damage },
    }
}
