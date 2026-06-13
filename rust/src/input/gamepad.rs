use bevy::prelude::*;
use godot::classes::Input;

use crate::character::player::{Player, RPlayer2D};
use crate::{character::Aim, 
    //gamestate::start::StartGameMessage
    };

use godot::global::JoyAxis;
use godot::prelude::*;
use godot_bevy::prelude::*;
const SPEED: f32 = 300.0;
const DEADZONE: f32 = 0.2;
pub(crate) fn plugin(app: &mut App) {
    app//.add_message::<StartGameMessage>()
        .add_systems(Update, gamepad);//.run_if(any_with_component::<Player>));
}

fn gamepad(
    mut player_aim_query: Query<(&GodotNodeHandle, &mut Aim), With<Player>>,
    mut godot: GodotAccess,
    //mut start_game_message: MessageWriter<StartGameMessage>,
) {
    let Ok((controller, mut aim)) = player_aim_query.single_mut() else {
        info!("No Player");
        return;
    };
    let Some(mut player) = godot.try_get::<RPlayer2D>(*controller) else {
        info!("No Player in GODOT");
        return;
    };

    let gd_input = godot.singleton::<Input>();

    // aim
    let aim_x = gd_input.get_joy_axis(0, JoyAxis::RIGHT_X);
    let aim_y = gd_input.get_joy_axis(0, JoyAxis::RIGHT_Y);
    let aim_vec = Vec2::new(aim_x, aim_y);
    if aim_vec.length() > 0.2 {
        aim.vec = aim_vec;
    }

    let movement_x = gd_input.get_joy_axis(0, JoyAxis::LEFT_X);
    let movement_y = gd_input.get_joy_axis(0, JoyAxis::LEFT_Y);
    let mut movement_vec = Vector2::new(movement_x, movement_y);
    if movement_vec.length() < DEADZONE {
        movement_vec = Vector2::ZERO;
    }

    player.set_velocity(movement_vec * SPEED);
    
    // CharacterBody2D only moves once move_and_slide() applies the velocity
    player.move_and_slide();

    // TODO: Shoot

    // menu system
}
