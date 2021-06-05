#[derive(Copy, Clone, Debug)]
pub struct Cell {
    alive: bool,
    alive_prev: bool,
    draw: bool,    // redraw this cell this frame
    recheck: bool, // this is in the list to get rechecked next frame
}

impl Cell {
    pub fn is_alive(self) -> bool {
        self.alive
    }
}

#[derive(Clone, Copy)]
pub struct CellRef {
    pub x: usize,
    pub y: usize,
}

pub struct World {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    cells_to_recheck: Vec<CellRef>,
}

impl World {
    fn get_neighbors(&self, x: usize, y: usize) -> [(usize, usize); 8] {
        let minx = (x + self.width - 1) % self.width;
        let miny = (y + self.height - 1) % self.height;
        let maxx = (x + 1) % self.width;
        let maxy = (y + 1) % self.height;

        [
            (minx, miny),
            (minx, y),
            (minx, maxy),
            (x, miny),
            (x, maxy),
            (maxx, miny),
            (maxx, y),
            (maxx, maxy),
        ]
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        self.cells[y * self.width + x].alive
    }

    fn was_alive(&self, x: usize, y: usize) -> bool {
        self.cells[y * self.width + x].alive_prev
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    fn get_mut_cell(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y * self.width + x]
    }

    fn get_cloned_cell(&mut self, x: usize, y: usize) -> Cell {
        self.cells[y * self.width + x]
    }

    pub fn new(
        width: usize,
        height: usize,
        initializer: &dyn Initializer,
        visualizer: &mut dyn Visualizer,
    ) -> World {
        let mut cells = Vec::with_capacity(width * height);
        let mut cells_to_recheck = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let alive = initializer.initialize_cell(x, y);
                let cell = Cell {
                    alive: alive,
                    alive_prev: alive,
                    draw: true,
                    recheck: true,
                };
                cells.push(cell);
                cells_to_recheck.push(CellRef { x, y });
            }
        }
        let world = World {
            cells,
            width,
            height,
            cells_to_recheck,
        };
        visualizer.update_world(&world);
        world
    }

    fn reset_world(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_mut_cell(x, y);
                cell.draw = false;
                cell.recheck = false;
                cell.alive_prev = cell.alive;
            }
        }
    }

    pub fn iterate(&mut self, visualizer: &mut dyn Visualizer) {
        // println!("====== New Iteration");
        let mut cells_to_draw = Vec::new();
        let mut cells_to_recheck_next = Vec::new();
        self.reset_world();
        for cell_to_recheck in self.cells_to_recheck.to_vec().iter() {
            let x = cell_to_recheck.x;
            let y = cell_to_recheck.y;
            // println!("Checking ({}, {})", x, y);

            let neighbors = self.get_neighbors(x, y);
            let mut alive_neighbors = 0;
            let mut cell = self.get_cloned_cell(x, y);
            let mut recheck = false;
            for (nx, ny) in neighbors.iter() {
                if self.was_alive(*nx, *ny) {
                    alive_neighbors += 1;
                }
            }
            if alive_neighbors < 2 || alive_neighbors > 3 {
                cell.alive = false;
                if cell.alive_prev != cell.alive {
                    recheck = true;
                    cell.draw = true;
                }
            }
            if alive_neighbors == 3 {
                cell.alive = true;
                if cell.alive_prev != cell.alive {
                    recheck = true;
                    cell.draw = true;
                }
            }
            if cell.draw {
                // println!("Asking to draw ({}, {})", x, y);
                cells_to_draw.push(CellRef { x, y });
            }
            if recheck {
                if !cell.recheck {
                    cell.recheck = true;
                    cells_to_recheck_next.push(CellRef { x, y });
                }
                for (nx, ny) in neighbors.iter() {
                    let neighbor = self.get_mut_cell(*nx, *ny);
                    if !neighbor.recheck {
                        neighbor.recheck = true;
                        cells_to_recheck_next.push(CellRef { x: *nx, y: *ny });
                    }
                }
            }
            let mut_cell = self.get_mut_cell(x, y);
            *mut_cell = cell;
        }
        visualizer.update_cells(&self, &cells_to_draw);
        self.cells_to_recheck = cells_to_recheck_next;
    }
}

pub trait Visualizer {
    fn update_world(&mut self, world: &World) -> ();
    fn update_cells(&mut self, world: &World, cells: &Vec<CellRef>) -> ();
    fn start_loop(&mut self, world: &mut World);
}

pub trait Initializer {
    fn initialize_cell(&self, x: usize, y: usize) -> bool;
}
