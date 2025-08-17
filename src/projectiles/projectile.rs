
#[derive(Component)]
struct Projectile {
    damage: u32,
    sprite: Sprite
}

#[derive(Bundle)]
struct PlayerProjectileBundle {
    projectile: Projectile,
    speed: u32,
    player: Player,
    transform: Transform
}

#[derive(Bundle)]
struct EnemyProjectileBundle {
    projectile: Projectile,
    enemy: Enemy,
    transform: Transform
}
