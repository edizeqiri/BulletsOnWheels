use bevy::ecs::error::debug;
use crate::character::player::Player;
use crate::character::{Aim, ShootingState, player_collision_groups};
use crate::gamestate::GameState;
use crate::weapon::ShootEvent;
use bevy::input::gamepad::GamepadEvent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub(super) fn plugin(app: &mut App) {
    app
        .add_systems(Update, gamepad_start)
        .add_systems(
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
fn gamepad_start(
    mut gamepad_event: MessageReader<GamepadEvent>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in gamepad_event.read() {
        if let GamepadEvent::Button(button_event) = event {
            info!("Gamepad clicks button {:?}", button_event.button);
            if matches!(
                button_event.button,
                GamepadButton::Start | GamepadButton::Select | GamepadButton::West
            ) {
                match state.get() {
                    GameState::START => next_state.set(GameState::RUNNING),
                    _ => {}
                }
            }
        }
    }
}

    fn gamepad_aim(
    controller_query: Query<&Gamepad>,
    mut player_aim_query: Query<&mut Aim, With<Player>>,
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
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(controller) = controller_query.single()
        && let Ok(mut velocity) = player_query.single_mut()
    {
        velocity.linvel = controller.left_stick() * 200.0;
    }
}

fn gamepad_input_system(
    mut gamepad_event: MessageReader<GamepadEvent>,
    mut player_query: Query<&mut ShootingState, With<Player>>,
) {
    let Ok(mut shooting_state) = player_query.single_mut() else {
        return;
    };

    for event in gamepad_event.read() {
        if let GamepadEvent::Button(button_event) = event {
            if matches!(
                button_event.button,
                GamepadButton::East | GamepadButton::RightTrigger
            ) {
                shooting_state.is_shooting = button_event.state.is_pressed();
            }
        }
    }
}

fn shoot_system(
    mut event_writer: MessageWriter<ShootEvent>,
    player_query: Query<(Entity, &ShootingState), With<Player>>,
    mut shoot_timer: Local<f32>,
    time: Res<Time>,
) {
    let Ok((player, shooting_state)) = player_query.single() else {
        return;
    };

    *shoot_timer -= time.delta_secs();

    if shooting_state.is_shooting && *shoot_timer <= 0.0 {
        event_writer.write(ShootEvent {
            shooter: player,
            collision_groups: player_collision_groups(),
        });
        *shoot_timer = 0.01;
    }
}
