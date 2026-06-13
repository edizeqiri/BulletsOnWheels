use bevy::prelude::*;
use godot::classes::Input;

use crate::character::player::{Player, RPlayer2D};
use crate::{character::Aim, 
    //gamestate::start::StartGameMessage
    };

use godot::global::JoyAxis;
use godot::prelude::*;
use godot_bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app//.add_message::<StartGameMessage>()
        .add_systems(Update, gamepad.run_if(any_with_component::<Player>));
}

fn gamepad(
    mut player_aim_query: Query<(&GodotNodeHandle, &mut Aim), With<Player>>,
    mut godot: GodotAccess,
    //mut start_game_message: MessageWriter<StartGameMessage>,
) {
    let Ok((controller, mut aim)) = player_aim_query.single_mut() else {
        return;
    };
    let Some(mut player) = godot.try_get::<RPlayer2D>(*controller) else {
        return;
    };

    let input = godot.singleton::<Input>();

    // aim
    let aim_x = input.get_joy_axis(0, JoyAxis::RIGHT_X);
    let aim_y = input.get_joy_axis(0, JoyAxis::RIGHT_Y);
    let aim_vec = Vec2::new(aim_x, aim_y);
    if aim_vec.length() > 0.2 {
        aim.vec = aim_vec;
    }

    // movement
    let movement_x = input.get_joy_axis(0, JoyAxis::LEFT_X);
    let movement_y = input.get_joy_axis(0, JoyAxis::LEFT_Y);
    let movement_vec = Vector2::new(movement_x, movement_y);

    player.set_velocity(movement_vec);

    // TODO: Shoot

    // menu system
    
    if input.is_action_just_pressed("start") {
        //start_game_message.write(StartGameMessage {});
    }
}
