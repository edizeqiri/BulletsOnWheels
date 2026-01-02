use crate::terrain::level1;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(level1::plugin);
}
