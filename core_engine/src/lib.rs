use bevy::{prelude::*, state::app::StatesPlugin};

use crate::gamestate::{AppState, InGameState};

pub mod character;
pub mod enemy;
mod enemy_ai;
pub mod gamestate;
pub mod player;
pub mod projectile;
pub mod score;
pub mod weapon;
pub mod world;

pub fn plugin(app: &mut App) {
    app.add_plugins(world::plugin)
        .add_plugins(weapon::plugin)
        .add_plugins(character::plugin)
        .add_plugins(gamestate::plugin)
        .add_plugins(score::plugin)
       // .add_plugins(StatesPlugin)
        .init_state::<InGameState>()
        .init_state::<AppState>();
}
