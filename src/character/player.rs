use crate::weapon::Weapon;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups};
use crate::character::{collider, square_sprite, Health, ENEMY_GROUP, PLAYER_GROUP};
use crate::weapon::bow::Bow;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    health: Health,
    player: Player,
    weapon: Weapon,
    collider: Collider,
    collider_group: CollisionGroups,
    transform: Transform,
    sprite: Sprite,
}

pub fn create_player() -> PlayerBundle {
    PlayerBundle {
        health: Health { current: 0, max: 1 },
        player: Player,
        weapon: Weapon::Bow(Bow {}),
        collider: collider(),
        collider_group: CollisionGroups::new(PLAYER_GROUP, ENEMY_GROUP),
        transform: Default::default(),
        sprite: square_sprite(Color::Srgba(Srgba::BLUE)),
    }
}