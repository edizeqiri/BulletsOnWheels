use std::time::Duration;
use bevy::color::Color;
use bevy::color::palettes::basic::GREEN;
use bevy::log::info;
use bevy::prelude::{Commands, Component, EventReader, Query, Res, Timer, TimerMode, Transform};
use bevy::time::Time;
use crate::character::{player_collision_groups, square_sprite, Aim};
use crate::weapon::{ShootEvent, Weapons};
use crate::weapon::Shootable;

#[derive(Component, Clone)]
pub struct WeaponCooldown {
    pub timer: Timer,
}

#[derive(Component, Clone)]
pub struct WeaponCooldowns(pub Vec<WeaponCooldown>);

impl WeaponCooldowns {
    pub fn new(weapons: &Weapons) -> Self {
        Self(
            weapons.0
                .iter()
                .map(|weapon| WeaponCooldown::new(weapon.fire_rate()))
                .collect()
        )
    }
}

impl WeaponCooldown {
    pub fn new(fire_rate: f32) -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f32(1.0 / fire_rate),
                TimerMode::Once
            ),
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.timer.finished()
    }

    pub fn reset(&mut self) {
        self.timer.reset();
    }
}

pub(crate) fn update_weapon_cooldowns(
    time: Res<Time>,
    mut cooldown_query: Query<&mut WeaponCooldowns>,
) {
    for mut cooldowns in &mut cooldown_query {
        for cooldown in &mut cooldowns.0 {
            cooldown.timer.tick(time.delta());
        }
    }
}

pub(crate) fn shoot_on_event(
    mut commands: Commands,
    mut shoot_event: EventReader<ShootEvent>,
    mut shooter_query: Query<(&Weapons, &Aim, &mut WeaponCooldowns, &Transform)>,
) {
    for event in shoot_event.read() {
        if let Ok((weapons, aim, mut cooldowns, transform)) = shooter_query.get_mut(event.shooter) {
            // Zip weapons with their cooldowns
            for (weapon, cooldown) in weapons.0.iter().zip(cooldowns.0.iter_mut()) {
                if cooldown.can_shoot() {
                    commands.spawn((
                        weapon.shoot(aim.vec),
                        square_sprite(Color::Srgba(GREEN)),
                        player_collision_groups(),
                        transform.clone()
                    ));
                    cooldown.reset();
                    info!("Got ShootingEvent and Shot with {:?} ", weapon);
                }
            }
        }
    }
}