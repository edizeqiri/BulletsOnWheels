use bevy::prelude::*;

use crate::gamestate::InGameState;
use crate::level_manager::LevelId::MainMenu;
use crate::level_manager::LoadLevelMessage;
use crate::{level1, menu, level_manager};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(reset_game)
        .add_plugins(level1::plugin)
        .add_plugins(menu::plugin)
        .add_plugins(level_manager::plugin)
        .add_systems(Startup, init_world)
        ;
}

fn init_world(mut commands: Commands) {
    commands.trigger(LoadLevelMessage { level_id: MainMenu });
}

#[derive(Event, Debug, Clone)]
pub struct PauseGameEvent;

#[derive(Event, Debug, Clone)]
pub struct ExitPauseGameEvent;

#[derive(Event, Debug, Clone)]
pub struct RestartGameEvent;

fn reset_game(_: On<RestartGameEvent>, mut commands: Commands) {
    commands.trigger(LoadLevelMessage {
        level_id: crate::level_manager::LevelId::MainMenu
    });
    commands.set_state(InGameState::RUNNING);
}

#[derive(Debug, Event)]
pub struct ResetSceneEvent(pub Entity);