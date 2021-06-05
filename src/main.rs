pub mod initializer;
pub mod sdlvisualizer;
pub mod world;

use initializer::OpenSimplexInitializer;
use sdlvisualizer::SdlVisualizer;
use world::Visualizer;
use world::World;

const WIDTH: usize = 1200;
const HEIGHT: usize = 900;

fn main() {
    let mut visualizer = SdlVisualizer::new(WIDTH, HEIGHT);
    let initializer = OpenSimplexInitializer::new();
    let mut world = World::new(WIDTH, HEIGHT, &initializer, &mut visualizer);
    visualizer.start_loop(&mut world);
}
