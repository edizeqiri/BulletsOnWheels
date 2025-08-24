use bevy::math::Vec2;
use crate::projectile::{create_projectile, ProjectileBundle};
use crate::weapon::Shootable;

#[derive(Debug, Copy, Clone)]
pub struct Gun {
    damage: u32,
    speed: f32,
    fire_rate: f32
}

impl Shootable for Gun {
    fn shoot(&self, direction: Vec2) -> ProjectileBundle {
        create_projectile(self.damage, self.speed, direction)
    }
    fn fire_rate(&self) -> f32 {
        self.fire_rate
    }
}

impl Gun {
    pub fn new(damage: u32, speed: f32,fire_rate: f32) -> Self {
        Self {
            damage,
            speed,
            fire_rate
        }
    }
}