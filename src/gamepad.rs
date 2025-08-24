use crate::character::player::Player;
use crate::character::Aim;
use crate::weapon::ShootEvent;
use bevy::input::gamepad::GamepadEvent;
use bevy::prelude::*;

/// Simple resource to store the ID of the first connected gamepad.
/// We can use it to know which gamepad to use for player input.

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            //gamepad_connections,
            //gamepad_input_events,
            gamepad_aim,
            gamepad_shoot.run_if(any_with_component::<Player>),
        ),
    );
}

fn gamepad_aim(controller_query: Query<&Gamepad>, player_aim_query: Query<&mut Aim, With<Player>>) {
    if let Ok(controller) = controller_query.single() {
        if let Ok(mut player_aim) = player_aim_query.single() {
            player_aim = &Aim {
                vec: controller.right_stick(),
            };
        }
    }
}

fn gamepad_shoot(
    mut gamepad_event: EventReader<GamepadEvent>,
    mut event_writer: EventWriter<ShootEvent>,
    player_aim_query: Query<Entity, With<Player>>,
) {
    let Ok(player) = player_aim_query.single() else {
        return; // No player or multiple players, skip this frame
    };

    for event in gamepad_event.read() {
        if let GamepadEvent::Button(button_event) = event {
            if let GamepadButton::East = button_event.button {
                event_writer.write(ShootEvent{shooter: player});
            }
        }
    }
}
