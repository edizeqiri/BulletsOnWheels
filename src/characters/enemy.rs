#[derive(Bundle)]
struct EnemyBundle {
    health: Health,
    enemy: Enemy,
    transform: Transform,
    sprite: Sprite,
}



#[derive(Component)]
struct Enemy;
