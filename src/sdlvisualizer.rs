use crate::world::{CellRef, Visualizer, World};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point, render::Canvas, Sdl};

pub struct SdlVisualizer {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    canvas: Canvas<sdl2::video::Window>,
    sdl_context: Sdl,
}

impl SdlVisualizer {
    pub fn new(width: usize, height: usize) -> SdlVisualizer {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let mut framerate_manager = sdl2::gfx::framerate::FPSManager::new();

        framerate_manager.set_framerate(200).unwrap();
        println!("Framerate: {}", framerate_manager.get_framerate());

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
    fn start_loop(&mut self, world: &mut World) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
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