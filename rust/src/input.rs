use bevy::app::App;

use crate::{gamepad, keyboard, mouse, virtual_joystick};

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(keyboard::plugin)
        .add_plugins(gamepad::plugin)
        //.add_plugins(mouse::plugin)
        .add_plugins(virtual_joystick::plugin);
}
