use crate::world::Initializer;
use noise::{NoiseFn, OpenSimplex, Seedable};

use crate::WIDTH;

pub struct UniformInitializer {
    threshold: f64,
}

impl UniformInitializer {
    pub fn new(threshold: f64) -> UniformInitializer {
        UniformInitializer { threshold }
    }
}

impl Initializer for UniformInitializer {
    fn initialize_cell(&self, _x: usize, _y: usize) -> bool {
        rand::random::<f64>() < self.threshold
    }
}

pub struct OpenSimplexInitializer {
    generator: OpenSimplex,
    lower_bound: f64,
    upper_bound: f64,
}

impl OpenSimplexInitializer {
    pub fn new() -> OpenSimplexInitializer {
        let generator = OpenSimplex::new().set_seed(rand::random::<u32>());
        OpenSimplexInitializer {
            generator,
            lower_bound: 0.25,
            upper_bound: 0.75,
        }
    }
}

impl OpenSimplexInitializer {
    fn map_generator_value(&self, n: f64) -> f64 {
        let n = 1.0 - n / 2.0;
        let n = n * (1.0 - self.lower_bound - (1.0 - self.upper_bound));
        n + self.lower_bound
    }
}

impl Initializer for OpenSimplexInitializer {
    fn initialize_cell(&self, x: usize, y: usize) -> bool {
        let p: f64 = (7 * x) as f64 / WIDTH as f64;
        let q: f64 = (7 * y) as f64 / WIDTH as f64;

        let threshold = self.map_generator_value(self.generator.get([p, q]));
        rand::random::<f64>() < threshold
    }
}
