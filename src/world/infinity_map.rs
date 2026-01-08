use bevy::prelude::*;

use crate::world::map::{Interpolator, Map, Path, PathStrategy, Strategy, VertexGenerator};

pub struct InfiniteMap {
    paths: Vec<Path>,
    size: u32,
    pub(crate) strategy: PathStrategy,
    buffered_paths: u32
}

impl Map for InfiniteMap {
    fn get_strategy(&mut self) -> &dyn Strategy {
        &self.strategy
    }

    fn get_paths(&mut self) -> &mut Vec<Path> {
        &mut self.paths
    }
}

struct InfinityVertex;
impl VertexGenerator for InfinityVertex {
    fn generate(&self, _start: Vec2, size: u32) -> Vec<Vec2> {
        todo!()
    }
}

struct InfinityInterpolator;
impl Interpolator for InfinityInterpolator {
    fn interpolate(&self, _vertices: &[Vec2]) -> Vec<Vec2> {
        todo!()
    }
}

// refactor
impl Default for InfiniteMap {
    fn default() -> Self {
        let vertex_generator: Box<InfinityVertex> = Box::new(InfinityVertex);
        let interpolator: Box<InfinityInterpolator> = Box::new(InfinityInterpolator);
        let path_strategy: PathStrategy = PathStrategy::new(vertex_generator, interpolator);
        let mut map = InfiniteMap::default();
        map.strategy = path_strategy;
        map
    }
}
