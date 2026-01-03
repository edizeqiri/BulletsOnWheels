use crate::character::player::PlayerDeathMessage;
use crate::gamestate::{EnemyResource, GameState, PlayerResource};
use crate::weapon::Weapons;
use bevy::app::{App, Update};
use bevy::prelude::{Commands, Name, OnEnter};
use rand::{Rng, rng};
use std::ops::Range;
use std::process::Command;

// --------------- PLAYER RESOURCES --------------- //
#[cfg(debug_assertions)]
pub const PLAYER_DEFAULTS: PlayerResource = PlayerResource { max_health: 10 };

#[cfg(not(debug_assertions))]
pub const PLAYER_DEFAULTS: PlayerResource = PlayerResource { max_health: 50 };

pub fn apply_player_defaults(mut commands: Commands) {
    commands.insert_resource(PLAYER_DEFAULTS);
}

// --------------- ENEMY RESOURCES --------------- //

#[cfg(debug_assertions)]
pub const ENEMY_DEFAULTS: EnemyResource = EnemyResource { max_health: 10 };

#[cfg(not(debug_assertions))]
pub const ENEMY_DEFAULTS: EnemyResource = EnemyResource { max_health: 50 };

pub fn apply_enemy_defaults(mut commands: Commands) {
    commands.insert_resource(ENEMY_DEFAULTS);
}
