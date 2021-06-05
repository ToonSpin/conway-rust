pub mod sdlvisualizer;
pub mod world;

use sdlvisualizer::SdlVisualizer;
use world::Visualizer;
use world::World;

const WIDTH: usize = 1200;
const HEIGHT: usize = 900;

fn initialize_cell(_x: usize, _y: usize) -> bool {
    rand::random::<f32>() < 0.25
}

fn main() {
    let mut visualizer = SdlVisualizer::new(WIDTH, HEIGHT);
    let mut world = World::new(WIDTH, HEIGHT, initialize_cell, &mut visualizer);
    visualizer.start_loop(&mut world);
}
