use bevy::prelude::App;

pub(crate) mod level1;
mod map;
mod walls;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(walls::plugin).add_plugins(map::plugin);
}
