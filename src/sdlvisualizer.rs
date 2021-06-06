use crate::world::{CellRef, Visualizer, World};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point, render::Canvas, Sdl};
// use std::time::Instant;

const FRAMERATE_SAMPLE_SIZE: u128 = 100;

pub struct SdlVisualizer {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    canvas: Canvas<sdl2::video::Window>,
    sdl_context: Sdl,
}

impl SdlVisualizer {
    fn draw_cell(&mut self, x: usize, y: usize) {
        let cx = x as i32;
        let cy = y as i32;

        if self.cells[y * self.width + x] {
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        } else {
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        }

        self.canvas.draw_point(Point::new(cx, cy)).unwrap();
    }

    fn draw_world(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.draw_cell(x, y);
            }
        }
    }
}

impl Visualizer for SdlVisualizer {
    fn new(width: Option<usize>, height: Option<usize>) -> SdlVisualizer {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let width = width.unwrap_or(1200);
        let height = height.unwrap_or(900);

        let window = video_subsystem
            .window("Game of Life", width as u32, height as u32)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        SdlVisualizer {
            width,
            height,
            cells: vec![false; width * height],
            canvas,
            sdl_context,
        }
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn start_loop(&mut self, world: &mut World) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut ticks = 0;
        // let mut prev_instant = Instant::now();

        'mainloop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'mainloop,
                    _ => {}
                }
            }

            world.iterate(self);
            self.canvas.present();

            if ticks == FRAMERATE_SAMPLE_SIZE {
                ticks = 0;
                // let millis = prev_instant.elapsed().as_millis();
                // println!("{} FPS", 1000 * FRAMERATE_SAMPLE_SIZE / millis);
                // prev_instant = Instant::now();
                self.draw_world();
            }
            ticks += 1;
        }
    }

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
            self.draw_cell(x, y);
        }
    }
}
