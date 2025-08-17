
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
