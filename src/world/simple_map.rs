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
        &self.strategy
    }

    fn get_paths(&mut self) -> &mut Vec<Path> {
        &mut self.paths
    }
}

#[derive(Default)]
pub struct SimpleVertex;
impl VertexGenerator for SimpleVertex {
    fn generate(&self, _start: Vec2, size: u32) -> Vec<Vec2> {
        let mut vertices: Vec<Vec2> = Vec::new();
        let mut rng = rand::rng();
        let scale = size as f32 * 100.;
        for x in 0..size {
            vertices.push(Vec2::new(
                rng.random_range(x as f32..scale),
                rng.random_range(x as f32..scale)
            ));
        }
        vertices
    }
}

#[derive(Default)]
pub struct SimpleInterpolator;
impl Interpolator for SimpleInterpolator {
    fn interpolate(&self, _vertices: &[Vec2]) -> Vec<Vec2> {
        let mut result = Vec::new();
        let mut paths: Vec<Vec2> = Vec::new();
        for x in 1.._vertices.len() {
            paths.insert(x - 1, _vertices[x] - _vertices[x - 1]);
        }

        for x in 0.._vertices.len() - 1 {
            let len_of_path: f32 = _vertices[x].length();
            for y in 0..(len_of_path / 20.) as i32 {
                result.push(_vertices[x] + paths[x] * (y as f32 / (len_of_path / 20.)));
            }
        }

        result
    }
}
