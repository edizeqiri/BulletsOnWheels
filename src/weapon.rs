use bevy::math::Vec2;
use bevy::prelude::Component;
use enum_dispatch::enum_dispatch;
use crate::projectile::{ProjectileBundle};
use crate::weapon::bow::Bow;

pub mod bow;

#[enum_dispatch(Shootable)]
#[derive(Component)]
pub enum Weapon {
    Bow(Bow),
}

#[enum_dispatch]
pub trait Shootable {
    fn shoot(&self, speed: f32, direction: Vec2) -> ProjectileBundle;
}

