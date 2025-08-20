use bevy::log::info;
use crate::projectile::{create_projectile, ProjectileBundle};
use crate::weapon::Shootable;

#[derive(Debug)]
pub struct Bow{}

impl Shootable for Bow {
    fn shoot(&self) -> ProjectileBundle {
        info!("Shooting Bow!");
        create_projectile(1.0,1.0)
    }
}
