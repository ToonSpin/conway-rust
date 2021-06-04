pub mod world;

use world::World;

const WIDTH: usize = 16;
const HEIGHT: usize = 16;

fn initialize_cell(_x: usize, _y: usize) -> bool {
    rand::random::<f32>() < 0.25
}

fn main() {
    let world = World::new(WIDTH, HEIGHT, initialize_cell);
    println!("{:?}", world);
}
