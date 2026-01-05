use std::time::Duration;

use bevy::color::palettes::basic::GREEN;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionGroups;

use crate::character::{Aim, square_sprite};
use crate::projectile::{ProjectileBundle, create_projectile};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<ShootEvent>()
        .add_systems(Update, (update_weapon_cooldowns, shoot_on_event));
}
#[derive(Copy, Clone, Debug)]
pub enum WeaponKind {
    Bow,
    Gun
}

#[derive(Clone, Debug)]
pub struct Weapon {
    kind: WeaponKind,
    damage: u32,
    speed: f32,
    fire_rate: f32,
    pub timer: Timer
}

impl Weapon {
    pub fn new(kind: WeaponKind, damage: u32, speed: f32, mut fire_rate: f32) -> Self {
        if fire_rate == 0. {
            fire_rate = 1.
        }
        Self {
            kind,
            damage,
            speed,
            fire_rate,
            timer: Timer::new(Duration::from_secs_f32(1.0 / fire_rate), TimerMode::Once)
        }
    }

    pub(crate) fn shoot(&self, direction: Vec2) -> ProjectileBundle {
        create_projectile(self.damage, self.speed, direction)
    }

    pub fn can_shoot(&self) -> bool {
        self.timer.is_finished()
    }

    pub fn reset(&mut self) {
        self.timer.reset();
    }
}

#[derive(Message)]
pub struct ShootEvent {
    pub shooter: Entity,
    pub collision_groups: CollisionGroups
}

#[derive(Component, Clone)]
pub struct Weapons {
    pub list: Vec<Weapon>
}

pub(crate) fn shoot_on_event(
    mut commands: Commands,
    mut shoot_event: MessageReader<ShootEvent>,
    mut shooter_query: Query<(&mut Weapons, &Aim, &Transform)>
) {
    for event in shoot_event.read() {
        if let Ok((mut weapons, aim, transform)) = shooter_query.get_mut(event.shooter) {
            for weapon in &mut weapons.list {
                if weapon.can_shoot() {
                    commands.spawn((
                        weapon.shoot(aim.vec),
                        square_sprite(Color::Srgba(GREEN)),
                        event.collision_groups,
                        *transform
                    ));
                    weapon.reset();
                }
            }
        }
    }
}

pub(crate) fn update_weapon_cooldowns(time: Res<Time>, mut weapon_query: Query<&mut Weapons>) {
    for mut weapons in &mut weapon_query {
        for weapon in &mut weapons.list {
            weapon.timer.tick(time.delta());
        }
    }
}

impl Default for Weapons {
    fn default() -> Self {
        Weapons {
            list: vec![
                Weapon::new(WeaponKind::Bow, 1, 1000., 0.5),
                Weapon::new(WeaponKind::Gun, 1, 250., 5.),
            ]
        }
    }
}
