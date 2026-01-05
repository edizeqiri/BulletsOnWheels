use bevy::prelude::*;
use glam::Vec2; // Better to use existing math types

pub struct Path {
    vertices: Vec2,
    points: Vec2
}

pub trait VertexGenerator {
    fn generate(&self, start: Vec2, config: &GenerationConfig) -> Vec2;
}

pub trait Interpolator {
    fn interpolate(&self, vertices: &Vec2) -> Vec2;
}

pub trait NoiseApplier {
    fn apply(&self, points: &mut Vec2);
}

pub struct PathStrategy {
    vertex_gen: Box<dyn VertexGenerator>,
    interpolator: Box<dyn Interpolator>,
    noise: Option<Box<dyn NoiseApplier>>
}
pub struct GenerationConfig {
    pub size: u32,
    pub vertex_count: u32
}

/// Trait for the map abstraction.
/// [GenerationConfig] is the chosen config for all the map strategies.
/// Every strategy can choose their own way of creating a [Path].
/// The idea would be that each [Path] in a [Map] is subset of the space in
/// which the [MapStrategy] can create paths. In other words, a [Map] should be
/// as cohesive as possible.
pub trait MapStrategy {
    fn build(&self, start: Vec2, config: &GenerationConfig) -> Path;
}

impl MapStrategy for PathStrategy {
    fn build(&self, start: Vec2, config: &GenerationConfig) -> Path {
        let vertices = self.vertex_gen.generate(start, config);

        let mut points = self.interpolator.interpolate(&vertices);

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

pub struct InfiniteMap {
    paths: Vec<Path>,
    config: GenerationConfig
}

pub trait Map {
    fn new(config: GenerationConfig) -> Self;
    fn add_path(&mut self, strategy: &dyn MapStrategy, start: Vec2);
}

impl Map for InfiniteMap {
    fn new(config: GenerationConfig) -> Self {
        Self {
            paths: Vec::new(),
            config
        }
    }

    fn add_path(&mut self, strategy: &dyn MapStrategy, start: Vec2) {
        let path = strategy.build(start, &self.config);
        self.paths.push(path);
    }
}
