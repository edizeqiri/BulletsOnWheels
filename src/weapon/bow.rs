use crate::projectile::{ProjectileBundle, create_projectile};
use crate::weapon::Shootable;
use bevy::math::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Bow {
    damage: u32,
    speed: f32,
    fire_rate: f32,
}

impl Shootable for Bow {
    fn shoot(&self, direction: Vec2) -> ProjectileBundle {
        create_projectile(self.damage, self.speed, direction)
    }
    fn fire_rate(&self) -> f32 {
        self.fire_rate
    }
}

impl Bow {
    pub fn new(damage: u32, speed: f32, fire_rate: f32) -> Self {
        Self {
            damage,
            speed,
            fire_rate,
        }
    }
}
