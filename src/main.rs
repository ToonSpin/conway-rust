const WIDTH: usize = 16;
const HEIGHT: usize = 16;

#[derive(Copy, Clone, Debug)]
struct Cell {
    alive: bool,
    alive_prev: bool,
    draw: bool,    // redraw this cell this frame
    recheck: bool, // this is in the list to get rechecked next frame
}

#[derive(Debug)]
struct World {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl World {
    fn new(width: usize, height: usize, value_func: fn (usize, usize) -> bool) -> World {
        let mut cells = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let alive = value_func(x, y);
                let cell = Cell {
                    alive: alive,
                    alive_prev: alive,
                    draw: true,
                    recheck: true,
                };
                cells.push(cell);
            }
        }
        World {
            cells,
            width,
            height,
        }
    }
}

fn initialize_cell(_x: usize, _y: usize) -> bool {
    rand::random::<f32>() < 0.25
}

fn main() {
    let world = World::new(WIDTH, HEIGHT, initialize_cell);
    println!("{:?}", world);
}
