use bevy::app::App;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::Event;
use bevy::log::info;
use bevy::prelude::{Message, Res, State, States};

pub(super) fn plugin(app: &mut App) {
    app;
    //.add_systems(FixedUpdate, log_state);
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    LOADING, // asset loading & level transition
    #[default]
    RUNNING, // character movement
    PAUSE,   // menu
    EXIT,    // exit
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum InGameState {
    PAUSED,
    #[default]
    RUNNING,
    DEFEAT,
}

#[derive(Message)]
pub struct CharacterDeathMessage {
    pub source: Entity,
    pub target: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct ExitGameEvent;

fn log_state(appstate: Res<State<AppState>>, ingamestate: Res<State<InGameState>>) {
    info!("appstate is: {:?}", appstate.get());
    info!("ingamestate is: {:?}", ingamestate.get());
}
