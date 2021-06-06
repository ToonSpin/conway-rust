use crate::world::Initializer;
use noise::{NoiseFn, OpenSimplex, Seedable};

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
    factor: f64,
}

impl OpenSimplexInitializer {
    pub fn new(width: usize, height: usize, lower_bound: f64, upper_bound: f64) -> OpenSimplexInitializer {
        let generator = OpenSimplex::new().set_seed(rand::random::<u32>());
        let factor = if width > height {
            7.0 / width as f64
        } else {
            7.0 / height as f64
        };
        OpenSimplexInitializer {
            generator,
            lower_bound,
            upper_bound,
            factor,
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
        let p: f64 = self.factor * x as f64;
        let q: f64 = self.factor * y as f64;

        let threshold = self.map_generator_value(self.generator.get([p, q]));
        rand::random::<f64>() < threshold
    }
}
