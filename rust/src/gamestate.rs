use bevy::app::{App, AppExit, FixedUpdate, Update};
use bevy::ecs::entity::Entity;
use bevy::log::info;
use bevy::prelude::{Message, MessageReader, MessageWriter, Res, State, States};
use godot_bevy::prelude::SceneTreeRef;

use crate::level_manager::LevelId;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<ExitGameMessage>()
        .add_systems(Update, exit_game);
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

pub trait GameStateTransition {
    fn current_state(&self) -> AppState;
    fn next_state(&self) -> AppState;
}

// ---------- MESSAGES CHANGING GAMESTATE --------- //

#[derive(Message)]
pub struct CharacterDeathMessage {
    pub source: Entity,
    pub target: Entity
}

// ---------- STATE MACHINE ---------- //

#[derive(Message)]
pub struct GameStateMessage {
    kind: GameStateEnum
}

#[derive(Copy, Clone)]
pub enum GameStateEnum {
    PlayerDeath,
    StartGame
}

impl From<&CharacterDeathMessage> for GameStateEnum {
    fn from(_message: &CharacterDeathMessage) -> Self {
        GameStateEnum::PlayerDeath
    }
}

pub fn aggregate_message_system<M>(
    mut messages: MessageReader<M>,
    mut writer: MessageWriter<GameStateMessage>
) where
    M: Message,
    for<'a> &'a M: Into<GameStateEnum>
{
    for message in messages.read() {
        writer.write(GameStateMessage {
            kind: message.into()
        });
    }
}

#[derive(Message)]
pub struct ExitGameMessage;

fn exit_game(
    mut exit_game_reader: MessageReader<ExitGameMessage>,
    mut exit: MessageWriter<AppExit>,
    mut scene_tree: SceneTreeRef
) {
    for _ in exit_game_reader.read() {
        info!("Exit game.");

        // bevy exit
        exit.write(AppExit::Success);

        // godot exit
        scene_tree.get().quit();
    }
}

fn log_state(
    appstate: Res<State<AppState>>,
    ingamestate: Res<State<InGameState>>,
    levlestate: Res<State<LevelId>>
) {
    info!("appstate is: {:?}", appstate.get());
    info!("ingamestate is: {:?}", ingamestate.get());
    info!("levlestate is: {:?}", levlestate.get());
}
