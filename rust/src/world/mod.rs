use bevy::prelude::*;

use crate::world::level::level1;
use crate::world::level_manager::*;
mod level;
pub(crate) mod level_manager;
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(level1::plugin)
        .add_plugins(level_manager::LevelManagerPlugin);
}
