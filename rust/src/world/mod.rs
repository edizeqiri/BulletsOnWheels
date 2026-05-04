use bevy::prelude::*;

use crate::gamestate::GameState;
use crate::world::level::{level0, level1, level2};
use crate::world::map::map::Level;

mod level;
pub mod map;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<LevelState>()
        .add_plugins(level0::plugin)
        .add_plugins(level1::plugin)
        .add_plugins(level2::plugin)
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
