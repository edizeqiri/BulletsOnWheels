use bevy::app::App;

use crate::score;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(score::plugin);
}
