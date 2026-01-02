use crate::character;
use crate::character::{ENEMY_GROUP, PLAYER_GROUP, square_sprite};
use crate::weapon::Weapons;
use bevy::color::palettes::css::BLUE;
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionGroups;

#[derive(Component)]
pub struct Player;

pub fn create_player_bundle(
    transform: Transform,
    weapons: Weapons,
    max_health: u32,
    name: &'static str,
) -> impl Bundle {
    (
        Name::new(name),
        character::create_character(transform, weapons, max_health),
        CollisionGroups::new(PLAYER_GROUP, ENEMY_GROUP),
        Player,
        square_sprite(Color::Srgba(BLUE)),
    )
}
