use crate::character::player::Player;
use crate::character::{Aim, ShootingState, player_collision_groups};
use crate::weapon::ShootEvent;
use bevy::input::gamepad::GamepadEvent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

/// Simple resource to store the ID of the first connected gamepad.
/// We can use it to know which gamepad to use for player input.

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        ((
            gamepad_aim,
            gamepad_input_system,
            shoot_system,
            gamepad_movement,
        )
            .run_if(any_with_component::<Player>),),
    );
}

fn gamepad_aim(
    controller_query: Query<&Gamepad>,
    mut player_aim_query: Query<&mut Aim, With<Player>>,
) {
    if let Ok(controller) = controller_query.single() {
        if let Ok(mut player_aim) = player_aim_query.single_mut() {
            if controller.right_stick().length() > 0.2 {
                player_aim.vec = controller.right_stick();
            }
        }
    }
}

fn gamepad_movement(
    controller_query: Query<&Gamepad>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(controller) = controller_query.single() {
        if let Ok(mut velocity) = player_query.single_mut() {
            velocity.linvel = controller.left_stick() * 200.0;
        }
    }
}

fn gamepad_input_system(
    mut gamepad_event: EventReader<GamepadEvent>,
    mut player_query: Query<&mut ShootingState, With<Player>>,
) {
    let Ok(mut shooting_state) = player_query.get_single_mut() else {
        return;
    };

    for event in gamepad_event.read() {
        if let GamepadEvent::Button(button_event) = event {
            if let GamepadButton::East | GamepadButton::RightTrigger = button_event.button {
                shooting_state.is_shooting = button_event.state.is_pressed();
            }
        }
    }
}

fn shoot_system(
    mut event_writer: EventWriter<ShootEvent>,
    player_query: Query<(Entity, &ShootingState), With<Player>>,
    mut shoot_timer: Local<f32>,
    time: Res<Time>,
) {
    let Ok((player, shooting_state)) = player_query.get_single() else {
        return;
    };

    *shoot_timer -= time.delta_secs();

    if shooting_state.is_shooting && *shoot_timer <= 0.0 {
        event_writer.send(ShootEvent {
            shooter: player,
            collision_groups: player_collision_groups(),
        });
        *shoot_timer = 0.01;
    }
}
fn gamepad_debug(mut gamepad_event: EventReader<GamepadEvent>) {
    for event in gamepad_event.read() {
        match event {
            GamepadEvent::Button(button_event) => {
                let button = button_event.button;
                let action = if button_event.state.is_pressed() {
                    "pressed"
                } else {
                    "released"
                };
                info!("Button {action}: {:?}", button);
            }
            GamepadEvent::Axis(axis_event) => {
                // Only log significant movements (avoid noise)
                if axis_event.value.abs() > 0.5 {
                    match axis_event.axis {
                        GamepadAxis::LeftStickX => {
                            info!("Left stick X: {:.2}", axis_event.value);
                        }
                        GamepadAxis::LeftStickY => {
                            info!("Left stick Y: {:.2}", axis_event.value);
                        }
                        GamepadAxis::RightStickX => {
                            info!("Right stick X: {:.2}", axis_event.value);
                        }
                        GamepadAxis::RightStickY => {
                            info!("Right stick Y: {:.2}", axis_event.value);
                        }
                        _ => {} // Ignore other axes (triggers, etc.)
                    }
                }
            }
            _ => {} // Ignore other event types
        }
    }
}
