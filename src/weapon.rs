use crate::projectile::ProjectileBundle;
use crate::weapon::bow::Bow;
use crate::weapon::cooldown::{shoot_on_event, update_weapon_cooldowns};
use crate::weapon::gun::Gun;
use bevy::math::Vec2;
use bevy::prelude::*;
use enum_dispatch::enum_dispatch;

pub mod bow;
pub mod cooldown;
pub mod gun;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<ShootEvent>()
        .add_systems(Update, (update_weapon_cooldowns, shoot_on_event));
}

#[enum_dispatch(Shootable)]
#[derive(Component, Copy, Clone, Debug)]
pub enum Weapon {
    Bow(Bow),
    Gun(Gun),
}

#[enum_dispatch]
pub trait Shootable {
    fn shoot(&self, direction: Vec2) -> ProjectileBundle;
    fn fire_rate(&self) -> f32;
}

#[derive(Event)]
pub struct ShootEvent {
    pub shooter: Entity,
}

#[derive(Component, Clone)]
pub struct Weapons(pub Vec<Weapon>);
