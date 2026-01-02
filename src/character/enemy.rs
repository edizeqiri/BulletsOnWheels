use crate::character;
use crate::character::{ENEMY_GROUP, PLAYER_GROUP, square_sprite};
use crate::weapon::Weapons;
use crate::weapon::cooldown::WeaponCooldowns;
use bevy::color::palettes::basic::RED;
use bevy::prelude::*;
use bevy_rapier2d::geometry::CollisionGroups;

#[derive(Component)]
pub struct Enemy;

pub fn create_enemy_bundle(
    transform: Transform,
    weapons: Weapons,
    max_health: u32,
    name: String,
) -> impl Bundle {
    (
        Name::new(name),
        character::create_character(transform, weapons.clone(), max_health),
        WeaponCooldowns::new(&weapons),
        CollisionGroups::new(ENEMY_GROUP, PLAYER_GROUP),
        Enemy,
        square_sprite(Color::Srgba(RED)),
    )
}
