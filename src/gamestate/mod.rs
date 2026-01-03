use crate::character::player::PlayerDeathMessage;
use bevy::app::App;
use bevy::prelude::{
    in_state, IntoScheduleConfigs, Message, MessageReader, MessageWriter, NextState, Res,
    ResMut, Resource, State, States, Update,
};
pub(crate) mod start;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<GameStateMessage>()
        .add_systems(Update, state_machine_system)
        .add_systems(
            Update,
            aggregate_message_system::<PlayerDeathMessage>.run_if(in_state(GameState::RUNNING)),
        );
}

// ---------- GAME STATE ---------- //

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    START,
    RUNNING,
    PAUSE,
    STOP,
}

// ---------- STATE MACHINE ---------- //

#[derive(Message)]
pub struct GameStateMessage {
    kind: GameStateEnum,
}

pub enum GameStateEnum {
    PlayerDeath,
}

impl From<&PlayerDeathMessage> for GameStateEnum {
    fn from(msg: &PlayerDeathMessage) -> Self {
        GameStateEnum::PlayerDeath
    }
}

pub fn aggregate_message_system<M>(
    mut messages: MessageReader<M>,
    mut writer: MessageWriter<GameStateMessage>,
) where
    M: Message,
    for<'a> &'a M: Into<GameStateEnum>,
{
    for message in messages.read() {
        writer.write(GameStateMessage {
            kind: message.into(),
        });
    }
}

fn state_machine_system(
    mut messages: MessageReader<GameStateMessage>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for message in messages.read() {
        match &message.kind {
            GameStateEnum::PlayerDeath => match current_state.get() {
                GameState::RUNNING => next_state.set(GameState::STOP),
                _ => {}
            },
        }
    }
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
