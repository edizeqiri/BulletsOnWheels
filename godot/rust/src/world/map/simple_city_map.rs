use std::collections::VecDeque;

use bevy::prelude::Component;
use glam::Vec2;
use rand::Rng;

use crate::world::map::map::{Map, Path, PathStrategy, Strategy, VertexGenerator};

#[derive(Component, Default)]
pub struct SimpleCityMap {
    paths: Vec<Path>,
    size: f32,
    strategy: SimpleStrategy
}

impl Map for SimpleCityMap {
    fn get_strategy(&mut self) -> &dyn Strategy {
        &self.strategy
    }

    fn get_paths(&mut self) -> &mut Vec<Path> {
        &mut self.paths
    }
}

struct SimpleStrategy {
    pub(crate) vertex_gen: Box<dyn VertexGenerator + Send + Sync>
}

impl Default for SimpleStrategy {
    fn default() -> Self {
        SimpleStrategy {
            vertex_gen: Box::new(SimpleCityVertex)
        }
    }
}

impl Strategy for SimpleStrategy {
    fn build(&self, start: Vec2, size: f32) -> Path {
        let points: Vec<Vec2> = self.vertex_gen.generate(start, size);
        Path {
            vertices: vec![],
            points
        }
    }
}
#[derive(Default)]
pub struct SimpleCityVertex;

impl VertexGenerator for SimpleCityVertex {
    /// In this current form, only driving upwards will be the intention of a
    /// new map.
    /// 1. From [start] generate a wall of possible points, like a 16x16 sprite
    ///    size times 100
    /// 2. based on the size go up
    /// 3. (Optional) rotate to fit need of player
    fn generate(&self, start: Vec2, mut size: f32) -> Vec<Vec2> {
        let mut vertices: Vec<Vec2> = Vec::new();
        let mut rng = rand::rng();
        let left_to_start = ((-size / 2.) as u32..(start.x - 64.) as u32).step_by(16);
        let right_to_start = ((start.x + 64.) as u32..(size / 2.) as u32).step_by(16);

        let mut queue: VecDeque<Vec2> = VecDeque::new();
        queue.push_back(start);

        while !queue.is_empty() && size > 0. {
            if let Some(tmp) = queue.pop_front() {
                let direction1 = rng.random_range(1..3);
                let direction2 = rng.random_range(1..3);
                let len = rng.random_range(100..1000);

                fn direction_map(random: i32) -> Vec2 {
                    match random {
                        1 => Vec2::new(1., 0.),
                        2 => Vec2::new(0., 1.),
                        3 => Vec2::new(-1., 0.),
                        _ => Vec2::new(0., 0.)
                    }
                }

                let dir_vec1 = direction_map(direction1);
                let dir_vec2 = direction_map(direction2);

                let new_vert1 = tmp + dir_vec1 * len as f32;
                queue.push_back(new_vert1);
                size -= 1.0;
                if direction1 != direction2 {
                    let new_vert2 = tmp + dir_vec2 * len as f32;
                    queue.push_back(new_vert2);
                    size -= 1.0;
                }

                vertices.append(&mut make_straight_walls(tmp, dir_vec1, len, 16));
                vertices.append(&mut make_straight_walls(tmp, dir_vec2, len, 16));
            };
        }

        // init layer
        for i in left_to_start {
            vertices.push(Vec2::new(i as f32, start.y));
        }

        for i in right_to_start {
            vertices.push(Vec2::new(i as f32, start.y));
        }

        for i in (-size / 2.) as u32..(size / 2.) as u32 {
            vertices.push(Vec2::new(i as f32, start.y));
        }
        vertices
    }
}

fn make_straight_walls(start: Vec2, dir: Vec2, len: u32, block_size: u32) -> Vec<Vec2> {
    let mut walls: Vec<Vec2> = Vec::new();
    let turned = dir.perp().normalize();
    let rev_turned = -turned;
    let mut i = 0;
    while i < len {
        walls.push(start + dir * i as f32 + turned * block_size as f32 * 4.);
        walls.push(start + dir * i as f32 + rev_turned);

        i += block_size;
    }
    walls
}
