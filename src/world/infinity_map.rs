use glam::Vec2;

use crate::world::map::{Interpolator, Map, Path, PathStrategy, Strategy, VertexGenerator};
#[derive(Default)]
pub struct InfiniteMap {
    paths: Vec<Path>,
    size: u32,
    strategy: PathStrategy
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
    fn generate(&self, start: Vec2) -> Vec<Vec2> {
        todo!()
    }
}

struct InfinityInterpolator;
impl Interpolator for InfinityInterpolator {
    fn interpolate(&self, vertices: &[Vec2]) -> Vec<Vec2> {
        todo!()
    }
}

// refactor
pub fn generate_map() -> InfiniteMap {
    let vertex_generator: Box<InfinityVertex> = Box::new(InfinityVertex);
    let interpolator: Box<InfinityInterpolator> = Box::new(InfinityInterpolator);
    let path_strategy: PathStrategy = PathStrategy::new(vertex_generator, interpolator);
    let mut map = InfiniteMap::default();
    map.strategy = path_strategy;
    map
}
