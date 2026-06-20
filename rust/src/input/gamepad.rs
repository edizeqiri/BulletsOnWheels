use bevy::prelude::*;
use godot::classes::Input;
use godot::global::JoyAxis;
use godot_bevy::prelude::*;

use crate::character::player::Player;
use crate::gamestate::ExitGameMessage;
use crate::weapon::weapon::ShootMessage;
use crate::{
    character::{Aim, MovementDirection} // gamestate::start::StartGameMessage
};

const DEADZONE: f32 = 0.05;

pub(crate) fn plugin(app: &mut App) {
    app //.add_message::<StartGameMessage>()
        .add_systems(Update, gamepad_input);
}

fn gamepad_input(
    mut query: Query<(&mut Aim, &mut MovementDirection, Entity), With<Player>>,
    mut godot: GodotAccess,
    mut shoot_message: MessageWriter<ShootMessage>,
    mut exit_game_message_writer: MessageWriter<ExitGameMessage>
) {
    let Ok((mut aim, mut movement, player)) = query.single_mut() else {
        return;
    };

    let gd_input = godot.singleton::<Input>();

    if gd_input.get_connected_joypads().is_empty() {
        return;
    };

    let aim_vec = Vec2::new(
        gd_input.get_joy_axis(0, JoyAxis::RIGHT_X),
        gd_input.get_joy_axis(0, JoyAxis::RIGHT_Y)
    );
    if aim_vec.length() > DEADZONE {
        aim.vec = aim_vec;
    }

    let move_vec = Vec2::new(
        gd_input.get_joy_axis(0, JoyAxis::LEFT_X),
        gd_input.get_joy_axis(0, JoyAxis::LEFT_Y)
    );
    movement.vec = if move_vec.length() < DEADZONE {
        Vec2::ZERO
    } else {
        move_vec
    };

    if gd_input.is_action_just_pressed("shoot") {
        shoot_message.write(ShootMessage {
            shooter: player,
            aim: Aim { vec: aim.vec }
        });
    };

    if gd_input.is_action_just_pressed("exit") {
        exit_game_message_writer.write(ExitGameMessage);
    };
}
