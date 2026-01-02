use crate::character;
use crate::character::{square_sprite, ENEMY_GROUP, PLAYER_GROUP};
use crate::weapon::cooldown::WeaponCooldowns;
use crate::weapon::Weapons;
use bevy::color::palettes::basic::RED;
use bevy::prelude::*;
use bevy_rapier2d::geometry::CollisionGroups;

#[derive(Bundle)]
pub struct EnemyBundle {
    collision_groups: CollisionGroups,
    enemy: Enemy,
    sprite: Sprite,
}

pub fn enemy_additions() -> EnemyBundle {
    EnemyBundle {
        collision_groups: CollisionGroups::new(ENEMY_GROUP, PLAYER_GROUP),
        enemy: Enemy,
        sprite: square_sprite(Color::Srgba(RED)),
    }
}

#[derive(Component)]
pub struct Enemy;

pub fn create_enemy_bundle(
    transform: Transform,
    weapons: Weapons,
    weapon_cooldowns: WeaponCooldowns,
    max_health: u32,
    name: &'static str,
) -> impl Bundle {
    (
        Name::new(name),
        character::create_character(transform, weapons, max_health),
        weapon_cooldowns.clone(),
        CollisionGroups::new(ENEMY_GROUP, PLAYER_GROUP),
        Enemy,
        square_sprite(Color::Srgba(RED)),
    )
}
