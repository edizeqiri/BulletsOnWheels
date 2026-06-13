use std::str::FromStr;

use bevy::math::Vec2;
use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::character::Aim;
use crate::weapon::Damage;
use crate::weapon::projectile::{ProjectileBundle, create_projectile};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<ShootEvent>();
}

#[derive(Component, GodotNode, Default, Clone)]
#[godot_node(base(RigidBody2D), class_name(RWeapon2D))]
pub struct Weapon {
    #[export_fields(value(export_type(f32), default(1.)))]
    damage: Damage,
    #[export_fields(value(export_type(f32), default(1.)))]
    speed: Speed,
    #[export_fields(value(export_type(f32), default(0.)))]
    fire_rate: FireRate,
    #[export_fields(value(export_type(WeaponKind), default(WeaponKind::GUN)))]
    weapon_kind: WeaponKindComponent,
}

#[derive(GodotConvert, Var, Export, Default, Clone)]
#[godot(via = GString)] // provides enum as string
pub enum WeaponKind {
    #[default]
    GUN,
    BOW,
    STAFF
}
#[derive(Component, Debug, Clone, Default, Copy)]
pub struct Speed(pub f32);
#[derive(Component, Debug, Clone, Default)]
pub struct FireRate(pub f32);
#[derive(Component, Clone, Default)]
pub struct WeaponKindComponent(pub WeaponKind);

impl Weapon {
    pub fn new(damage: f32, speed: f32, fire_rate: f32, weapon_kind: WeaponKind) -> Self {
        Self {
            damage: Damage(damage),
            speed: Speed(speed),
            fire_rate: FireRate(fire_rate),
            weapon_kind: WeaponKindComponent(weapon_kind),
        }
    }
    pub(crate) fn shoot(&self, direction: Vec2) -> ProjectileBundle {
        create_projectile(self.damage, self.speed, direction)
    }
}

#[derive(Message)]
pub struct ShootEvent {
    pub shooter: Entity,
}

#[derive(Component, Clone)]
pub struct Weapons {
    pub list: Vec<Weapon>,
}

pub(crate) fn shoot_on_event(
    mut commands: Commands,
    mut shoot_event: MessageReader<ShootEvent>,
    mut shooter_query: Query<(&mut Weapons, &Aim, &Transform)>,
) {
    for event in shoot_event.read() {
        if let Ok((mut weapons, aim, transform)) = shooter_query.get_mut(event.shooter) {
            for weapon in &mut weapons.list {
                commands.spawn((weapon.shoot(aim.vec), *transform));
            }
        }
    }
}

impl Default for Weapons {
    fn default() -> Self {
        Weapons {
            list: vec![
                Weapon::new(1., 1000., 0.5, WeaponKind::BOW),
                Weapon::new(1., 250., 5., WeaponKind::GUN),
            ],
        }
    }
}
