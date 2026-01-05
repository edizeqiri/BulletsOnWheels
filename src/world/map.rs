use bevy::prelude::*;

pub trait VertexGenerator {
    fn generate(&self, start: Vec2) -> Vec<Vec2>;
}

pub trait Interpolator {
    fn interpolate(&self, vertices: &[Vec2]) -> Vec<Vec2>;
}

pub trait NoiseApplier {
    fn apply(&self, points: &mut [Vec2]);
}

pub struct GenerationConfig {
    pub size: u32,
    pub vertex_count: u32
}

impl GenerationConfig {
    pub(crate) fn new(size: u32, vertex_count: u32) -> Self {
        GenerationConfig { size, vertex_count }
    }
}
pub struct Path {
    vertices: Vec<Vec2>,
    points: Vec<Vec2>
}
/// Trait for the map abstraction
///
/// [GenerationConfig] is the chosen config for all the map strategies.
/// Every strategy can choose their own way of creating a [Path].
/// The idea would be that each [Path] in a [Map] is subset of the space in
/// which the [Strategy] can create paths. In other words, a [Map] should be
/// as cohesive as possible.
pub trait Strategy {
    fn build(&self, start: Vec2) -> Path;
}

pub struct PathStrategy {
    vertex_gen: Box<dyn VertexGenerator>,
    interpolator: Box<dyn Interpolator>,
    noise: Option<Box<dyn NoiseApplier>>
}
#[derive(Default)]
struct SimpleVertex;
impl VertexGenerator for SimpleVertex {
    fn generate(&self, _start: Vec2) -> Vec<Vec2> {
        todo!()
    }
}
#[derive(Default)]
struct SimpleInterpolator;
impl Interpolator for SimpleInterpolator {
    fn interpolate(&self, _vertices: &[Vec2]) -> Vec<Vec2> {
        todo!()
    }
}

impl Default for PathStrategy {
    fn default() -> Self {
        PathStrategy {
            vertex_gen: Box::new(SimpleVertex::default()),
            interpolator: Box::new(SimpleInterpolator::default()),
            noise: None
        }
    }
}

impl Strategy for PathStrategy {
    fn build(&self, start: Vec2) -> Path {
        let vertices: Vec<Vec2> = self.vertex_gen.generate(start);

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
    fn add_path(&mut self, start: Vec2) {
        let strategy = self.get_strategy().build(start);
        self.get_paths().push(strategy);
    }
}
