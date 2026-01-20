use crate::agent::agent::Agent;
use crate::world::grid::Grid;
use crate::config::sim_config::Config;

pub struct Engine {
    agents: Vec<Agent>,
    world: Grid,
    generation: u32,
    config: Config,
}

impl Engine {
    pub fn new(config: Config) -> Self {
        let world = Grid::new(config.grid_width, config.grid_height);
        let agents = Vec::new();
        Engine {
            agents,
            world,
            generation: 0,
            config,
        }
    }

    pub fn print_grid(&self) {
        self.world.print();
    }
}