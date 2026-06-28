use bevy::prelude::*;
use godot::classes::Input;
use godot::global::JoyAxis;
use godot_bevy::prelude::*;

use crate::character::{Aim, MovementDirection};
use crate::gamestate::ExitGameMessage;
use crate::player::Player;
use crate::weapon_impl::ShootMessage;

pub const DEADZONE: f32 = 0.05;

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

    let move_vec = gd_input.get_vector("left", "right", "up", "down");

    let aim_vec_godot = gd_input.get_vector("aim_left", "aim_right", "aim_up", "aim_down");

    let aim_vec = Vec2 {
        x: aim_vec_godot.x,
        y: aim_vec_godot.y
    };

    if aim_vec.length() < DEADZONE {
        aim.vec = Vec2::ZERO;
    } else {
        aim.vec = aim_vec;
    }

    movement.vec = if move_vec.length() < DEADZONE {
        Vec2::ZERO
    } else {
        Vec2 {
            x: move_vec.x,
            y: move_vec.y
        }
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
