pub mod world;

use world::CellRef;
use world::Visualizer;
use world::World;

const WIDTH: usize = 16;
const HEIGHT: usize = 16;

struct NullVisualizer {}

impl Visualizer for NullVisualizer {
    fn update_world(&self, _: &World) {}
    fn update_cells(&self, _: &Vec<CellRef>) {}
}

fn initialize_cell(_x: usize, _y: usize) -> bool {
    rand::random::<f32>() < 0.25
}

fn main() {
    let visualizer = NullVisualizer {};
    let mut world = World::new(WIDTH, HEIGHT, initialize_cell, &mut visualizer);
    loop {
        world.iterate(&mut visualizer);
    }
    // println!("{:?}", world);
}
