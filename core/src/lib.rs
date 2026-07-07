use bevy::prelude::*;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(world::plugin)
        .add_plugins(weapon::plugin)
        .add_plugins(character::plugin)
        .add_plugins(gamestate::plugin)
        .add_plugins(score::plugin)
        .add_plugins(StatesPlugin)
        .init_state::<AppState>()
        .init_state::<InGameState>()
        .init_state::<LevelId>();
}
