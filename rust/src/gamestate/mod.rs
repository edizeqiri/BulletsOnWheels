use bevy::{app::{App, AppExit}, prelude::{
    Message, MessageReader, MessageWriter, NextState, Res, ResMut, Resource, State, States
}};
use godot_bevy::prelude::SceneTreeRef;

use crate::gamestate::start::StartGameMessage;

pub(crate) mod start;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<ExitGameMessage>();
}

// pub(super) fn plugin(app: &mut App) {
// app.add_message::<GameStateMessage>()
// .add_systems(Update, state_machine_system)
// .add_systems(
// Update,
// aggregate_message_system::<PlayerDeathMessage>.
// run_if(in_state(GameState::RUNNING)), )
// .add_systems(
// Update,
// aggregate_message_system::<StartGameMessage>.
// run_if(in_state(GameState::START)), );
// }
// ---------- GAME STATE ---------- //

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub(crate) enum GameState {
    #[default]
    START,
    RUNNING,
    PAUSE,
    STOP
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

/*
impl From<&PlayerDeathMessage> for GameStateEnum {
    fn from(_message: &PlayerDeathMessage) -> Self {
        GameStateEnum::PlayerDeath
    }
}
*/

impl From<&StartGameMessage> for GameStateEnum {
    fn from(_message: &StartGameMessage) -> Self {
        GameStateEnum::StartGame
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

// TODO: Refactor with enum delegate
fn state_machine_system(
    mut messages: MessageReader<GameStateMessage>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for message in messages.read() {
        match (message.kind, current_state.get()) {
            (GameStateEnum::PlayerDeath, GameState::RUNNING) => {
                next_state.set(GameState::STOP);
            },
            (GameStateEnum::StartGame, GameState::START) => {
                next_state.set(GameState::RUNNING);
            },
            _ => {}
        }
    }
}

#[derive(Resource, Clone)]
pub struct PlayerResource {
    // x_range: Range<i32>,
    // y_range: Range<i32>,
    // pub weapons: Weapons,
    pub max_health: u32
}

#[derive(Resource, Clone)]
pub struct EnemyResource {
    // x_range: Range<i32>,
    // y_range: Range<i32>,
    // pub weapons: Weapons,
    pub max_health: u32
}

#[derive(Message)]
pub struct ExitGameMessage;

fn exit_game(
    mut exit_game_reader: MessageReader<ExitGameMessage>,
    mut exit: MessageWriter<AppExit>,
    mut scene_tree: SceneTreeRef
) {
    for _ in exit_game_reader.read() {
        // bevy exit
        exit.write(AppExit::Success);

        // godot exit
        scene_tree.get().quit();
    }
}