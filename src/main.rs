pub mod initializer;
pub mod sdlvisualizer;
pub mod terminalvisualizer;
pub mod world;

use initializer::OpenSimplexInitializer;
use initializer::UniformInitializer;
use sdlvisualizer::SdlVisualizer;
use structopt::StructOpt;
use terminalvisualizer::TerminalVisualizer;
use world::Initializer;
use world::Visualizer;
use world::World;

#[derive(Debug, StructOpt)]
struct Options {
    /// The number of cells in the game area along the horizontal axis
    /// (default value depends on the visualizer)
    #[structopt(short = "W", long)]
    width: Option<usize>,

    /// The number of cells in the game area along the vertical axis
    /// (default value depends on the visualizer)
    #[structopt(short = "H", long)]
    height: Option<usize>,

    /// How the game is displayed
    #[structopt(short, long, possible_values=&["terminal", "sdl2"], default_value="sdl2")]
    visualizer: String,

    /// How the game area starts out
    #[structopt(short, long, possible_values=&["opensimplex","uniform"], default_value="opensimplex")]
    initializer: String,

    /// Sets the threshold for the "uniform" initializer
    #[structopt(short, long, default_value="0.25")]
    threshold: f64,

    /// Sets the threshold for the "opensimplex" initializer
    #[structopt(short, long, default_value="0.25")]
    lower_bound: f64,

    /// Sets the threshold for the "opensimplex" initializer
    #[structopt(short, long, default_value="0.75")]
    upper_bound: f64,
}

impl Options {
    fn get_visualizer(&self) -> Box<dyn Visualizer> {
        match self.visualizer.as_str() {
            "terminal" => Box::new(TerminalVisualizer::new(self.width, self.height)),
            "sdl2" => Box::new(SdlVisualizer::new(self.width, self.height)),
            _ => unreachable!(),
        }
    }

    fn get_initializer(&self, width: usize, height: usize) -> Box<dyn Initializer> {
        match self.initializer.as_str() {
            "opensimplex" => Box::new(OpenSimplexInitializer::new(width, height, self.lower_bound, self.upper_bound)),
            "uniform" => Box::new(UniformInitializer::new(self.threshold)),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let options = Options::from_args();

    let mut visualizer = options.get_visualizer();
    let width = visualizer.get_width();
    let height = visualizer.get_height();

    let mut initializer = options.get_initializer(width, height);
    let mut world = World::new(width, height, &mut (*initializer), &mut (*visualizer));

    visualizer.start_loop(&mut world);
}
