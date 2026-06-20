use bevy::prelude::*;
use godot::classes::Input;

use crate::character::player::Player;
use crate::{
    character::{Aim, MovementDirection},
    //gamestate::start::StartGameMessage
};

use godot::global::JoyAxis;
use godot_bevy::prelude::*;

const DEADZONE: f32 = 0.05;

pub(crate) fn plugin(app: &mut App) {
    app //.add_message::<StartGameMessage>()
        .add_systems(Update, gamepad_input);
}

fn gamepad_input(
    mut query: Query<(&mut Aim, &mut MovementDirection), With<Player>>,
    mut godot: GodotAccess,
) {
    let Ok((mut aim, mut movement)) = query.single_mut() else {
        return;
    };

    let gd_input = godot.singleton::<Input>();

    if gd_input.get_connected_joypads().is_empty() {
        return;
    };

    let aim_vec = Vec2::new(
        gd_input.get_joy_axis(0, JoyAxis::RIGHT_X),
        gd_input.get_joy_axis(0, JoyAxis::RIGHT_Y),
    );
    if aim_vec.length() > DEADZONE {
        aim.vec = aim_vec;
    }

    let move_vec = Vec2::new(
        gd_input.get_joy_axis(0, JoyAxis::LEFT_X),
        gd_input.get_joy_axis(0, JoyAxis::LEFT_Y),
    );
    movement.vec = if move_vec.length() < DEADZONE {
        Vec2::ZERO
    } else {
        move_vec
    };
}
