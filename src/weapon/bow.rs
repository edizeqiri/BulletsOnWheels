use crate::projectile::{ProjectileBundle, create_projectile};
use crate::weapon::Shootable;
use bevy::log::info;
use bevy::math::Vec2;

#[derive(Debug)]
pub struct Bow {}

impl Shootable for Bow {
    fn shoot(&self, speed: f32, direction: Vec2) -> ProjectileBundle {
        info!("Shooting Bow!");
        create_projectile(1, speed, direction)
    }
}
