mod gamepad;

use bevy::app::App;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(gamepad::plugin);
}
