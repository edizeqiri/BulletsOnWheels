use bevy::prelude::*;
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
        let mut last: Vec2 = _start;
        vertices.push(last);
        (0..size).for_each(|_x: u32| {
            let base_dir = Vec2::from_angle(rng.random_range(-1. ..1.));
            let dir = if last.length_squared() == 0.0 {
                base_dir
            } else {
                base_dir.rotate(last.normalize())
            };
            let next = dir * rng.random_range(10. ..scale);

            vertices.push(next + last);
            last += next;
        });
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
        for x in 0..paths.len() {
            let len_of_path: f32 = paths[x].length();
            for y in 0..(len_of_path / 20.) as i32 {
                let scaled_path = paths[x] * (y as f32 / (len_of_path / 20.));
                let scaled_inter_path = _vertices[x] + scaled_path;
                let tunred_90 = if y == 0 {
                    _vertices[x].normalize().perp()
                } else {
                    scaled_path.perp().normalize()
                } * 80.;
                if tunred_90.is_nan() {
                    continue;
                }
                result.push(tunred_90 + scaled_inter_path);
                result.push(-tunred_90 + scaled_inter_path);
                // result.push(scaled_inter_path);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::world::map::Interpolator;
    use crate::world::simple_map::SimpleInterpolator;

    #[test]
    fn interpolator_should_create_walls_around_points() {
        let vertices = [
            Vec2::new(0., 0.),
            Vec2::new(40., 40.),
            Vec2::new(120., 120.)
        ];
        let result = SimpleInterpolator::interpolate(&SimpleInterpolator, &vertices);

        assert_eq!(result.len(), 12);
    }
}
