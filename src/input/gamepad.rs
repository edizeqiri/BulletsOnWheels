use bevy::input::gamepad::GamepadEvent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::character::player::Player;
use crate::character::{Aim, ShootingState};
use crate::gamestate::start::StartGameMessage;
use crate::gamestate::GameState;

pub(crate) fn plugin(app: &mut App) {
    app.add_message::<StartGameMessage>()
        .add_systems(
            Update,
            ((gamepad_shoot_system, gamepad_aim, gamepad_movement)
                .run_if(any_with_component::<Player>),)
        )
        .add_systems(
            Update,
            gamepad_in_menu_system.run_if(in_state(GameState::START))
        );
}

fn gamepad_aim(
    controller_query: Query<&Gamepad>,
    mut player_aim_query: Query<&mut Aim, With<Player>>
) {
    if let Ok(controller) = controller_query.single()
        && let Ok(mut player_aim) = player_aim_query.single_mut()
        && controller.right_stick().length() > 0.2
    {
        player_aim.vec = controller.right_stick();
    }
}

fn gamepad_movement(
    controller_query: Query<&Gamepad>,
    mut player_query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(controller) = controller_query.single()
        && let Ok(mut velocity) = player_query.single_mut()
    {
        velocity.linvel = controller.left_stick() * 200.0;
    }
}

fn gamepad_shoot_system(
    mut gamepad_event: MessageReader<GamepadEvent>,
    mut player_query: Query<&mut ShootingState, With<Player>>
) {
    let Ok(mut shooting_state) = player_query.single_mut() else {
        return;
    };
    for event in gamepad_event.read() {
        if let GamepadEvent::Button(button_event) = event {
            info!("button {:?}", button_event.button);
            match button_event.button {
                GamepadButton::East | GamepadButton::RightTrigger => {
                    shooting_state.is_shooting = button_event.state.is_pressed();
                },
                _ => {}
            }
        }
    }
}

fn gamepad_in_menu_system(
    mut gamepad_event: MessageReader<GamepadEvent>,
    mut start_game_message: MessageWriter<StartGameMessage>
) {
    for event in gamepad_event.read() {
        if let GamepadEvent::Button(button_event) = event {
            info!("button {:?}", button_event.button);
            if let GamepadButton::Start = button_event.button {
                start_game_message.write(StartGameMessage {});
            }
        }
    }
}
