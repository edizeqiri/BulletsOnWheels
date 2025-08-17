#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    health: Health,
    player: Player,
    transform: Transform,
    sprite: Sprite,
}