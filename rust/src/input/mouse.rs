use bevy::prelude::*;
use godot_bevy::{plugins::input::MouseButton, prelude::*};

use crate::character::{Aim, player::Player};

pub(crate) fn plugin(app: &mut App) {
    app
        .add_systems(Update, handle_mouse_motion_system)
        .add_systems(Update, handle_mouse_system);
}

fn handle_mouse_motion_system(
    mut query: Query<&mut Aim, With<Player>>,
    mut events: MessageReader<MouseMotion>,
) {
    let Ok(mut aim) = query.single_mut() else {
        return;
    };
    
    for event in events.read() {
        aim.vec = event.position;
    }
}

fn handle_mouse_system(
    mut query: Query<&mut Aim, With<Player>>,
    mut events: MessageReader<MouseButtonInput>
) {
    let Ok(mut aim) = query.single_mut() else {
        return;
    };
    
    for event in events.read() {
        if event.pressed {
            match event.button {
                MouseButton::Left => aim.vec = event.position,
                _ => {}
            }
        }
    }
}
