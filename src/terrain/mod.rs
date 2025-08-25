use crate::terrain::walls::spawn_perimeter_walls;
use bevy::prelude::{App, Startup};

mod walls;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_perimeter_walls);
}
