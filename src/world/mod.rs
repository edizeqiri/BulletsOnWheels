use bevy::prelude::*;

mod infinity_map;
pub(crate) mod level1;
mod map;
mod simple_map;
pub mod walls;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(level1::plugin).add_plugins(map::plugin);
}
