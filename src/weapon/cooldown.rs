use crate::weapon::Shootable;
use crate::weapon::Weapons;
use bevy::prelude::{Component, Query, Res, Timer, TimerMode};
use bevy::time::Time;
use std::time::Duration;

#[derive(Component, Clone)]
pub struct WeaponCooldown {
    pub timer: Timer,
}

#[derive(Component, Clone)]
pub struct WeaponCooldowns(pub Vec<WeaponCooldown>);

impl WeaponCooldowns {
    pub fn new(weapons: &Weapons) -> Self {
        Self(
            weapons
                .0
                .iter()
                .map(|weapon| WeaponCooldown::new(weapon.fire_rate()))
                .collect(),
        )
    }
}

impl WeaponCooldown {
    pub fn new(fire_rate: f32) -> Self {
        Self {
            timer: Timer::new(Duration::from_secs_f32(1.0 / fire_rate), TimerMode::Once),
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
