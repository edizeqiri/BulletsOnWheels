mod gamepad;
mod keyboard;

use bevy::app::App;
use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((gamepad::plugin, keyboard::plugin));
}
