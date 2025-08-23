use crate::character::{square_sprite, ENEMY_GROUP, PLAYER_GROUP};
use bevy::color::palettes::css::BLUE;
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionGroups;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    collision_groups: CollisionGroups,
    player: Player,
    sprite: Sprite

}

pub fn player_additions() -> PlayerBundle {
    PlayerBundle {
        collision_groups: CollisionGroups::new(PLAYER_GROUP, ENEMY_GROUP),
        player: Player,
        sprite: square_sprite(Color::Srgba(BLUE))
    }
}