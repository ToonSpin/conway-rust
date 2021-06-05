pub mod world;
pub mod terminalvisualizer;

use world::World;
use terminalvisualizer::TerminalVisualizer;

const WIDTH: usize = 16;
const HEIGHT: usize = 16;

fn initialize_cell(_x: usize, _y: usize) -> bool {
    rand::random::<f32>() < 0.25
}

fn main() {
    let mut visualizer = TerminalVisualizer::new(WIDTH, HEIGHT);
    let mut world = World::new(WIDTH, HEIGHT, initialize_cell, &mut visualizer);
    loop {
        world.iterate(&mut visualizer);
    }
    // println!("{:?}", world);
}
