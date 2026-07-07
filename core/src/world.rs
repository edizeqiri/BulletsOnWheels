use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app;
}

#[derive(Event, Debug, Clone)]
pub struct PauseGameEvent;

#[derive(Event, Debug, Clone)]
pub struct ExitPauseGameEvent;

#[derive(Event, Debug, Clone)]
pub struct RestartGameEvent;
