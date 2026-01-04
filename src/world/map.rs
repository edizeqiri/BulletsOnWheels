use bevy::log::tracing::metadata::ParseLevelFilterError;
use crate::world::level1;
use bevy::prelude::*;
use bevy_rapier2d::rapier::prelude::Point;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(level1::plugin);
}


struct InfiniteMap {
    paths: Vec<InfinitePath>
}
impl Map for InfiniteMap {
    fn create(&self) {
        todo!()
    }
}
struct InfinitePath;

impl Vertex for InfinitePath {
    fn create(&self) {
        todo!()
    }
}

impl Interpolate for InfinitePath {
    fn interpolate(&self) {
        todo!()
    }
}

impl Path for InfinitePath {

}

trait Path: Vertex + Interpolate {
    // vertices: Vec2, // (X1,Y1),(X2,Y2)
    // points: Vec2
}

// Map -> 1:n Path -> Vertex
// Points
// 1. random linked vertices
// 2. interpolate to smooth
// 3. noise to path
trait Map {
    fn create(&self);

}
trait Vertex {
    fn create(&self);
}

trait Interpolate {
    fn interpolate(&self);
}
