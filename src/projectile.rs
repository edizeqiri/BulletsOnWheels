use bevy::prelude::*;
use crate::character::enemy::Enemy;
use crate::character::player;
use crate::character::player::Player;

#[derive(Component)]
struct Projectile {
    damage: u32,
    speed: u32,
    sprite: Sprite
}

#[derive(Bundle)]
struct PlayerProjectileBundle {
    projectile: Projectile,
    player: Player
}

#[derive(Bundle)]
struct EnemyProjectileBundle {
    projectile: Projectile,
    enemy: Enemy
}
