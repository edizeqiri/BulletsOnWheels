use bevy::prelude::{Resource, States};

pub(crate) mod start;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    START,
    RUNNING,
    STOP,
}

#[derive(Resource, Clone)]
pub struct PlayerResource {
    // x_range: Range<i32>,
    // y_range: Range<i32>,
    //pub weapons: Weapons,
    pub max_health: u32,
}

#[derive(Resource, Clone)]
pub struct EnemyResource {
    // x_range: Range<i32>,
    // y_range: Range<i32>,
    //pub weapons: Weapons,
    pub max_health: u32,
}
