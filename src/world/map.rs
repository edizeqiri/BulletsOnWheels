use bevy::prelude::*;

use crate::world::simple_map::{SimpleInterpolator, SimpleVertex};

pub trait VertexGenerator {
    fn generate(&self, start: Vec2, size: f32) -> Vec<Vec2>;
}

pub trait Interpolator {
    fn interpolate(&self, vertices: &[Vec2]) -> Vec<Vec2>;
}

pub trait NoiseApplier {
    fn apply(&self, points: &mut [Vec2]);
}
/// [vertices]: a [Vec2] of main points which act as joints or crossroads
/// [points]: a [Vec2] of the connections between the [vertices]. Can be viewed as points on the
/// path between two vertices
#[derive(Clone, Debug)]
pub struct Path {
    pub vertices: Vec<Vec2>,
    pub points: Vec<Vec2>
}
/// Trait for the map abstraction
///
/// [GenerationConfig] is the chosen config for all the map strategies.
/// Every strategy can choose their own way of creating a [Path].
/// The idea would be that each [Path] in a [Map] is subset of the space in
/// which the [Strategy] can create paths. In other words, a [Map] should be
/// as cohesive as possible.
pub trait Strategy {
    fn build(&self, start: Vec2, size: f32) -> Path;
}

pub struct PathStrategy {
    pub(crate) vertex_gen: Box<dyn VertexGenerator>,
    pub(crate) interpolator: Box<dyn Interpolator>,
    pub(crate) noise: Option<Box<dyn NoiseApplier>>
}

impl Default for PathStrategy {
    fn default() -> Self {
        PathStrategy {
            vertex_gen: Box::new(SimpleVertex),
            interpolator: Box::new(SimpleInterpolator),
            noise: None
        }
    }
}

impl Strategy for PathStrategy {
    fn build(&self, start: Vec2, size: f32) -> Path {
        let vertices: Vec<Vec2> = self.vertex_gen.generate(start, size);

        let mut points: Vec<Vec2> = self.interpolator.interpolate(&vertices);

        if let Some(noise) = &self.noise {
            noise.apply(&mut points);
        }

        Path { vertices, points }
    }
}
impl PathStrategy {
    pub fn new(vertex_gen: Box<dyn VertexGenerator>, interpolator: Box<dyn Interpolator>) -> Self {
        Self {
            vertex_gen,
            interpolator,
            noise: None
        }
    }

    pub fn with_noise(mut self, noise: Box<dyn NoiseApplier>) -> Self {
        self.noise = Some(noise);
        self
    }
}

pub trait Map {
    fn get_strategy(&mut self) -> &dyn Strategy;
    fn get_paths(&mut self) -> &mut Vec<Path>;
    /// A [Path] is a collection of vertices and points between the vertices.
    /// The size defines the amount of vertices and the spacing of the
    /// vertices. Concrete implementation may vary based on map type
    fn add_path(&mut self, start: Vec2, size: f32) -> Vec2 {
        let new_path = self.get_strategy().build(start, size);
        self.get_paths().push(new_path.clone());
        match new_path.points.last() {
            Some(x) => *x,
            _ => start
        }
    }
}

pub struct Level {
    pub(crate) goal: Vec2
}