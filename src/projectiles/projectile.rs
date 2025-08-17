
#[derive(Component)]
struct Projectile {
    damage: u32,
    sprite: Sprite
}

#[derive(Component)]
struct Speed(u32);

#[derive(Bundle)]
struct PlayerProjectileBundle {
    projectile: Projectile,
    speed: Speed,
    player: Player,
    transform: Transform
}

#[derive(Bundle)]
struct EnemyProjectileBundle {
    projectile: Projectile,
    enemy: Enemy,
    transform: Transform
}
