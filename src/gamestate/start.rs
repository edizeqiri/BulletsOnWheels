use std::ops::Range;
use bevy::prelude::Name;
use rand::{rng, Rng};
use crate::gamestate::EnemyProperties;
use crate::weapon::Weapons;

impl Default for EnemyProperties {
    fn default() -> Self {
        EnemyProperties {
            //weapons: Weapons::default(),
            max_health: 20,
            name: Name::from("Enemy")
        }
    }
}