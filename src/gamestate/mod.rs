use std::ops::Range;
use bevy::app::App;
use bevy::prelude::{Name, Resource};
use crate::weapon::Weapons;

mod start;


pub enum GameState {
    START,
    RUNNING,
    STOP
}

#[derive(Resource)]
pub struct EnemyProperties {
   // x_range: Range<i32>,
   // y_range: Range<i32>,
    //pub weapons: Weapons,
    pub max_health: u32,
    pub name: Name
}