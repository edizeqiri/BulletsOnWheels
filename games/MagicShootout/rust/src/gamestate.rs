use bevy::app::{App, AppExit, FixedUpdate, Update};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::Event;
use bevy::ecs::observer::On;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::log::info;
use bevy::prelude::{Message, MessageReader, MessageWriter, Res, State, States};

use crate::level_manager::LevelId;

pub(super) fn plugin(app: &mut App) {
    app;
    //.add_systems(FixedUpdate, log_state);
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub(crate) enum AppState {
    LOADING, // asset loading & level transition
    #[default]
    RUNNING, // character movement
    PAUSE,   // menu
    EXIT     // exit
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub(crate) enum InGameState {
    PAUSED,
    #[default]
    RUNNING,
    DEFEAT
}

#[derive(Message)]
pub struct CharacterDeathMessage {
    pub source: Entity,
    pub target: Entity
}

#[derive(Event, Debug, Clone)]
pub struct ExitGameEvent;


fn log_state(
    appstate: Res<State<AppState>>,
    ingamestate: Res<State<InGameState>>,
    levlestate: Res<State<LevelId>>
) {
    info!("appstate is: {:?}", appstate.get());
    info!("ingamestate is: {:?}", ingamestate.get());
    info!("levlestate is: {:?}", levlestate.get());
}
