use bevy::prelude::*;
use crate::character::Health;

#[derive(Bundle)]
struct EnemyBundle {
    health: Health,
    enemy: Enemy,
    transform: Transform,
    sprite: Sprite,
}


#[derive(Component)]
pub struct Enemy;
