use crate::agent::agent::Agent;
use crate::config::sim_config::SimConfig;
use crate::world::grid::Grid;

pub struct Engine {
    agents: Vec<Agent>,
    pub world: Grid,
    generation: u32,
    config: SimConfig,
}

impl Engine {
    pub fn new(config: SimConfig) -> Self {
        let world = Grid::new(config.grid_width, config.grid_height);
        let agents = Vec::new();
        Engine {
            agents,
            world,
            generation: 0,
            config: config,
        }
    }

    pub fn print_grid(&self) {
        self.world.print();
    }
}
