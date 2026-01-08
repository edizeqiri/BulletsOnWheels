use glam::Vec2;
use rand::Rng;

use crate::world::map::{Interpolator, Map, Path, PathStrategy, Strategy, VertexGenerator};

#[derive(Default)]
pub struct SimpleMap {
    paths: Vec<Path>,
    size: u32,
    strategy: PathStrategy
}

impl Map for SimpleMap {
    fn get_strategy(&mut self) -> &dyn Strategy {
        todo!()
    }

    fn get_paths(&mut self) -> &mut Vec<Path> {
        todo!()
    }
}

impl SimpleMap {
    pub fn new(strategy: PathStrategy) -> Self {
        SimpleMap {
            strategy,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct SimpleVertex;
impl VertexGenerator for SimpleVertex {
    fn generate(&self, _start: Vec2, size: u32) -> Vec<Vec2> {
        let mut vertices: Vec<Vec2> = Vec::new();
        let mut rng = rand::rng();

        for x in (0..size) {
            vertices.push(Vec2::new(
                rng.random_range(0. ..100.),
                rng.random_range(0. ..100.)
            ));
        }
        vertices
    }
}

#[derive(Default)]
pub struct SimpleInterpolator;
impl Interpolator for SimpleInterpolator {
    fn interpolate(&self, _vertices: &[Vec2]) -> Vec<Vec2> {
        todo!()
    }
}
