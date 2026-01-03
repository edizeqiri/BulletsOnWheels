use crate::character::player::PlayerDeathMessage;
use bevy::log::info;
use bevy::prelude::{Entity, Message, MessageReader, MessageWriter, Resource, States};
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

pub enum GameStateEnum {
    PlayerDeath { entity: Entity },
}

#[derive(Message)]
pub struct GameStateMessage {
    body: GameStateEnum,
}

impl From<&PlayerDeathMessage> for GameStateEnum {
    fn from(msg: &PlayerDeathMessage) -> Self {
        GameStateEnum::PlayerDeath { entity: msg.entity }
    }
}
pub fn game_state_aggregator<M>(
    mut messages: MessageReader<M>,
    mut writer: MessageWriter<GameStateMessage>,
) where
    M: Message,
    for<'a> &'a M: Into<GameStateEnum>,
{
    for message in messages.read() {
        writer.write(GameStateMessage {
            body: message.into(),
        });
    }
}

fn state_machine(mut messages: MessageReader<GameStateMessage>) {
    for message in messages.read() {
        match &message.body {
            PlayerDeath => info!("Penis"),
        }
    }
}
