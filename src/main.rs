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
    fn new(width: usize, height: usize) -> World {
        let mut cells = Vec::with_capacity(width * height);
        for _y in 0..height {
            for _x in 0..width {
                let cell = Cell {
                    alive: false,
                    alive_prev: false,
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

fn main() {
    let world = World::new(WIDTH, HEIGHT);
    println!("{:?}", world);
}
