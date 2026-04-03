use bevy::prelude::*;

use crate::character::Health;
use crate::character::enemy::Enemy;
use crate::gamestate::GameState;
use crate::world::levels::level0::generate_level0;
use crate::world::levels::{level0, level1};
use crate::world::map::{Level, Map};
use crate::world::walls::create_wall_bundle;

mod infinity_map;

mod levels;
pub mod map;
mod simple_map;
mod walls;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<LevelState>()
        .add_plugins(level0::plugin)
        .add_plugins(level1::plugin)
        .add_systems(OnEnter(GameState::START), (clean_up));
}

fn clean_up(mut command: Commands, level: Single<Entity, With<Level>>) {
    command.entity(level.entity()).despawn()
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum LevelState {
    #[default]
    NONE,
    ZERO,
    ONE,
    TWO
}
