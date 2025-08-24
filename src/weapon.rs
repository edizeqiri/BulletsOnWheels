use crate::character::{Aim, player_collision_groups, square_sprite};
use crate::projectile::ProjectileBundle;
use crate::weapon::bow::Bow;
use crate::weapon::cooldown::{WeaponCooldowns, update_weapon_cooldowns};
use crate::weapon::gun::Gun;
use bevy::color::palettes::basic::GREEN;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionGroups;
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
    pub collision_groups: CollisionGroups,
}

#[derive(Component, Clone)]
pub struct Weapons(pub Vec<Weapon>);

pub(crate) fn shoot_on_event(
    mut commands: Commands,
    mut shoot_event: EventReader<ShootEvent>,
    mut shooter_query: Query<(&Weapons, &Aim, &mut WeaponCooldowns, &Transform)>,
) {
    for event in shoot_event.read() {
        shoot_all_weapons(
            &mut commands,
            &mut shooter_query,
            event.shooter,
            event.collision_groups,
        );
    }
}

pub fn shoot_all_weapons(
    commands: &mut Commands,
    shooter_query: &mut Query<(&Weapons, &Aim, &mut WeaponCooldowns, &Transform)>,
    shooter: Entity,
    collision_groups: CollisionGroups,
) {
    if let Ok((weapons, aim, mut cooldowns, transform)) = shooter_query.get_mut(shooter) {
        // Zip weapons with their cooldowns
        for (weapon, cooldown) in weapons.0.iter().zip(cooldowns.0.iter_mut()) {
            if cooldown.can_shoot() {
                commands.spawn((
                    weapon.shoot(aim.vec),
                    square_sprite(Color::Srgba(GREEN)),
                    collision_groups,
                    transform.clone(),
                ));
                cooldown.reset();
                info!("Got ShootingEvent and Shot with {:?} ", weapon);
            }
        }
    }
}
