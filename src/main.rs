pub mod initializer;
pub mod sdlvisualizer;
pub mod terminalvisualizer;
pub mod world;

use initializer::OpenSimplexInitializer;
use sdlvisualizer::SdlVisualizer;
use structopt::StructOpt;
use terminalvisualizer::TerminalVisualizer;
use world::Visualizer;
use world::World;

#[derive(Debug, StructOpt)]
struct Options {
    /// The number of cells in the game area along the horizontal axis. Default
    /// value depends on the visualizer.
    #[structopt(short = "W", long)]
    width: Option<usize>,

    /// The number of cells in the game area along the vertical axis. Default
    /// value depends on the visualizer.
    #[structopt(short = "H", long)]
    height: Option<usize>,

    /// How the game is displayed.
    #[structopt(short, long, possible_values=&["terminal", "sdl2"], default_value="sdl2")]
    visualizer: String,
}

impl Options {
    fn get_visualizer(&self) -> Box<dyn Visualizer> {
        match self.visualizer.as_str() {
            "terminal" => Box::new(TerminalVisualizer::new(self.width, self.height)),
            "sdl2" => Box::new(SdlVisualizer::new(self.width, self.height)),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let options = Options::from_args();

    let mut visualizer = options.get_visualizer();
    let width = visualizer.get_width();
    let height = visualizer.get_height();

    let initializer = OpenSimplexInitializer::new(width, height);
    let mut world = World::new(width, height, &initializer, &mut (*visualizer));
    visualizer.start_loop(&mut world);
}
