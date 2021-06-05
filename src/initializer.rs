use crate::world::Initializer;

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
