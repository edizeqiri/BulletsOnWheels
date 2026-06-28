use bevy::app::App;
use crate::{keyboard, gamepad, mouse};

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(keyboard::plugin)
        .add_plugins(gamepad::plugin)
        .add_plugins(mouse::plugin);
}
