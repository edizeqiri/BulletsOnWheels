use crate::character::ShootingState;
use crate::character::player::Player;
use crate::gamestate::{EnemyResource, GameState, GameStateEnum, PlayerResource};
use bevy::app::App;
use bevy::input::gamepad::GamepadEvent;
use bevy::log::info;
use bevy::prelude::{
    Commands, GamepadButton, IntoScheduleConfigs, Message, MessageReader, MessageWriter, Query,
    Update, With, in_state,
};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<StartGameMessage>().add_systems(
        Update,
        gamepad_start_input_system.run_if(in_state(GameState::START)),
    );
}

#[derive(Message)]
pub struct StartGameMessage;

fn gamepad_start_input_system(
    mut gamepad_event: MessageReader<GamepadEvent>,
    mut start_game_message: MessageWriter<StartGameMessage>,
) {
    for event in gamepad_event.read() {
        if let GamepadEvent::Button(button_event) = event {
            info!("button {:?}", button_event.button);
            match button_event.button {
                GamepadButton::Start => {
                    start_game_message.write(StartGameMessage {});
                }
                _ => {}
            }
        }
    }
}

// --------------- PLAYER RESOURCES --------------- //
#[cfg(debug_assertions)]
pub const PLAYER_DEFAULTS: PlayerResource = PlayerResource { max_health: 10 };

#[cfg(not(debug_assertions))]
pub const PLAYER_DEFAULTS: PlayerResource = PlayerResource { max_health: 50 };

pub fn apply_player_defaults(mut commands: Commands) {
    commands.insert_resource(PLAYER_DEFAULTS);
}

// --------------- ENEMY RESOURCES --------------- //

#[cfg(debug_assertions)]
pub const ENEMY_DEFAULTS: EnemyResource = EnemyResource { max_health: 10 };

#[cfg(not(debug_assertions))]
pub const ENEMY_DEFAULTS: EnemyResource = EnemyResource { max_health: 50 };

pub fn apply_enemy_defaults(mut commands: Commands) {
    commands.insert_resource(ENEMY_DEFAULTS);
}
