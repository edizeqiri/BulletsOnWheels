use bevy::prelude::*;
use godot::global::Key;
use godot_bevy::prelude::*;

use crate::character::{Movement, player::Player};

pub(crate) fn plugin(app: &mut App) {
    app
        .add_systems(Update, handle_keyboard_system);
}

fn handle_keyboard_system(
    mut events: MessageReader<KeyboardInput>,
    mut query: Query<(&mut Movement), With<Player>>,
) {
    let Ok(mut movement) = query.single_mut() else {
        return;
    };
    
    for event in events.read() {
        if event.pressed {
            movement.vec = match event.keycode {
                Key::W => Vec2 { x: 0., y: 1. },
                Key::A => Vec2 { x: -1., y: 0. },
                Key::S => Vec2 { x: 0., y: -1. },
                Key::D => Vec2 { x: 1., y: 0. },
                _ => Vec2::ZERO
            };
        }
    }
}
