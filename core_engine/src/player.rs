use bevy::prelude::*;

use crate::character::{CharacterCore, Health, MovementSpeed};

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    health: Health,
    speed: MovementSpeed,
    core: CharacterCore,
}

#[derive(Message)]
pub struct PlayerDeathMessage {
    pub entity: Entity,
}
