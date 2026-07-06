use bevy::app::App;
use bevy::prelude::*;

use crate::world::ButtonEnteredEvent;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_mouse_enter_button_animation);
}

fn on_mouse_enter_button_animation(
    trigger: On<ButtonEnteredEvent>,
    godot: GodotAccess
) {
    let nine_patct_rect = trigger.button_handle.get_
}