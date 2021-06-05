pub mod world;

use world::CellRef;
use world::Visualizer;
use world::World;

const WIDTH: usize = 16;
const HEIGHT: usize = 16;

struct TerminalVisualizer {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl TerminalVisualizer {
    fn new(width: usize, height: usize) -> TerminalVisualizer {
        TerminalVisualizer {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    fn draw_world(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y * self.width + x] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

impl Visualizer for TerminalVisualizer {
    fn update_world(&mut self, world: &World) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.cells[y * self.width + x] = world.is_alive(x, y);
            }
        }
        self.draw_world();
    }
    fn update_cells(&mut self, world: &World, cells: &Vec<CellRef>) {
        for cell in cells.iter() {
            let x = cell.x;
            let y = cell.y;
            self.cells[y * self.width + x] = world.is_alive(x, y);
        }
        println!("{}", String::from('=').repeat(self.width));
        self.draw_world();
    }
}

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
