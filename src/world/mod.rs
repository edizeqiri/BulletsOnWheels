use bevy::prelude::*;

mod infinity_map;
pub(crate) mod level1;
pub mod map;
mod simple_map;
mod walls;
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(level1::plugin);
}
