use bevy::app::App;
use bevy::prelude::*;
use godot::classes::{Button, NinePatchRect, class_macros::private::virtuals::ZipReader::Vector2};
use godot_bevy::interop::GodotAccess;

use crate::world::{ButtonEnteredEvent, ButtonExitedEvent};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_mouse_enter_button_animation)
        .add_observer(on_mouse_exit_button_animation);
}

fn on_mouse_enter_button_animation(
    trigger: On<ButtonEnteredEvent>,
    mut godot: GodotAccess
) {
    let Some(button) = godot.try_get::<Button>(trigger.button_handle) else {
        info!("No button found for this handle.");
        return;
    };

    // sprite of the button
    let Some(mut nine_patch_rect) = button.try_get_node_as::<NinePatchRect>("RestartSprite") else {
        info!("No child found of type NinePatchRect");
        return;
    };

    // ANIMATE BUTTON
    // make brighter
    nine_patch_rect.set_modulate(godot::prelude::Color::from_rgb(1.2, 1.2, 1.2));
    // increase size of button
    let size = nine_patch_rect.get_size();
    nine_patch_rect.set_pivot_offset(size / 2.0);
    nine_patch_rect.set_scale(Vector2::new(1.05, 1.05));
}

fn on_mouse_exit_button_animation(
    trigger: On<ButtonExitedEvent>
    , mut godot: GodotAccess
) {
    let Some(button) = godot.try_get::<Button>(trigger.button_handle) else {
        info!("No button found for this handle.");
        return;
    };

    // sprite of the button
    let Some(mut nine_patch_rect) = button.try_get_node_as::<NinePatchRect>("RestartSprite") else {
        info!("No child found of type NinePatchRect");
        return;
    };

    // RESET ANIMATION
    nine_patch_rect.set_modulate(godot::prelude::Color::from_rgb(1.0, 1.0, 1.0));
    nine_patch_rect.set_scale(Vector2::new(1., 1.));
}