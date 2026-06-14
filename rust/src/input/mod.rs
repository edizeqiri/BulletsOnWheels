mod gamepad;
mod keyboard;
mod mouse;

use bevy::app::App;

pub(crate) fn plugin(app: &mut App) {
    app
        .add_plugins(gamepad::plugin)
        .add_plugins(keyboard::plugin)
        .add_plugins(mouse::plugin);
}
