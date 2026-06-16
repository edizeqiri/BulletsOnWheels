use bevy::{math::VectorSpace, prelude::*};
use godot::{global::Key, prelude::*};
use godot_bevy::prelude::*;
use godot::classes::Input;

use crate::character::{Movement, player::Player};

pub(crate) fn plugin(app: &mut App) {
    app
        .add_systems(Update, handle_keyboard_system);
}

fn handle_keyboard_system(
    mut events: MessageReader<KeyboardInput>,
    mut query: Query<(&mut Movement), With<Player>>,
    mut godot: GodotAccess
) {
    let Ok(mut movement) = query.single_mut() else {
        return;
    };

    let gd_input = godot.singleton::<Input>();

    let mut base = Vec2::ZERO;
    if gd_input.is_key_pressed(Key::W) { base += Vec2 { x: 0., y: -1. };}
    if gd_input.is_key_pressed(Key::A) { base += Vec2 { x: -1., y: 0. };}
    if gd_input.is_key_pressed(Key::S) { base += Vec2 { x: 0., y: 1. };}
    if gd_input.is_key_pressed(Key::D) { base += Vec2 { x: 1., y: 0. };}


    movement.vec = base.normalize_or_zero();

}
