use crate::weapon::Weapon;
use bevy::prelude::*;
use crate::character::{square_sprite, Health};
use crate::weapon::bow::Bow;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    health: Health,
    player: Player,
    weapon: Weapon,
    transform: Transform,
    sprite: Sprite,
}

pub fn create_player() -> PlayerBundle {
    PlayerBundle {
        health: Health { current: 0, max: 1 },
        player: Player,
        weapon: Weapon::Bow(Bow {}),
        transform: Default::default(),
        sprite: square_sprite(Color::Srgba(Srgba::BLUE)),
    }
}