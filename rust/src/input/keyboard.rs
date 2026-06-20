use bevy::prelude::*;
use godot::classes::Input;
use godot::global::Key;
use godot_bevy::prelude::*;

use crate::character::{MovementDirection, player::Player};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_keyboard_system);
}

fn handle_keyboard_system(mut query: Query<&mut MovementDirection, With<Player>>, mut godot: GodotAccess) {
    let Ok(mut movement) = query.single_mut() else {
        return;
    };

    let gd_input = godot.singleton::<Input>();

    let mut base = Vec2::ZERO;

    // Up
    if gd_input.is_key_pressed(Key::W) {
        base += Vec2 { x: 0., y: -1. };
    }
    // Left
    if gd_input.is_key_pressed(Key::A) {
        base += Vec2 { x: -1., y: 0. };
    }
    // Down
    if gd_input.is_key_pressed(Key::S) {
        base += Vec2 { x: 0., y: 1. };
    }
    // Right
    if gd_input.is_key_pressed(Key::D) {
        base += Vec2 { x: 1., y: 0. };
    }

    movement.vec = base.normalize_or_zero();
}
