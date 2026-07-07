use bevy::{ecs::world, prelude::*, state::app::StatesPlugin};

use crate::gamestate::{AppState, InGameState};

mod character;
mod enemy;
mod enemy_ai;
mod gamestate;
mod player;
mod projectile;
mod score;
mod weapon;
mod weapon_impl;
mod world;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(world::plugin)
        .add_plugins(weapon::plugin)
        .add_plugins(character::plugin)
        .add_plugins(gamestate::plugin)
        .add_plugins(score::plugin)
        .add_plugins(StatesPlugin)
        .init_state::<AppState>()
        .init_state::<InGameState>();
}
