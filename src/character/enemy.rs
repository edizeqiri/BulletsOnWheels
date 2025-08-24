use crate::character::{ENEMY_GROUP, PLAYER_GROUP, square_sprite};
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
