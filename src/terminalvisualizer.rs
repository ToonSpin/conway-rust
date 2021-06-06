use crate::world::{CellRef, Visualizer, World};

pub struct TerminalVisualizer {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl TerminalVisualizer {
    fn draw_world(&self) {
        let mut chars = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y * self.width + x] {
                    chars.push('#');
                } else {
                    chars.push(' ');
                }
            }
            chars.push('\n');
        }
        print!("{}", chars.iter().collect::<String>());
        println!("{}", String::from("-").repeat(self.width));
    }
}

impl Visualizer for TerminalVisualizer {
    fn new(width: Option<usize>, height: Option<usize>) -> TerminalVisualizer {
        let width = width.unwrap_or(80);
        let height = height.unwrap_or(30);
        TerminalVisualizer {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn start_loop(&mut self, world: &mut World) {
        loop {
            world.iterate(self);
            self.draw_world();
        }
    }

    fn update_world(&mut self, world: &World) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.cells[y * self.width + x] = world.is_alive(x, y);
            }
        }
    }

    fn update_cells(&mut self, world: &World, cells: &Vec<CellRef>) {
        for cell in cells.iter() {
            let x = cell.x;
            let y = cell.y;
            self.cells[y * self.width + x] = world.is_alive(x, y);
        }
    }
}
