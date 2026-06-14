use bevy::prelude::*;
use godot::{global::Key, prelude::*};
use godot_bevy::prelude::*;

use crate::character::{Movement, player::Player};

pub(crate) fn plugin(app: &mut App) {
    app
        .add_systems(Update, handle_keyboard_system);
}

fn handle_mouse(mut events: MessageReader<MouseButtonInput>) {
    for event in events.read() {
        println!(
            "Mouse button: {:?} at {:?}",
            event.button_index, event.position
        );
    }
}

fn handle_mouse_motion(mut events: MessageReader<MouseMotion>) {
    for event in events.read() {
        println!("Mouse moved by: {:?}", event.relative);
    }
}
