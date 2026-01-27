use crate::Coordinate;
use crate::agent::agent::Agent;
use crate::config::sim_config::SimConfig;
use crate::rng::RNG;
use crate::world::{
    grid::Grid,
    resource::{Resource, ResourceType},
};

#[derive(Clone, Debug)]
pub struct Engine {
    next_agent_id: u32,
    agents: Vec<Agent>,
    world: Grid,
    generation: u32,
    config: SimConfig,
    rng: RNG,
}

impl Engine {
    pub fn new(config: SimConfig) -> Self {
        let world = Grid::new(config.grid_width, config.grid_height);
        let agents = Vec::new();
        Engine {
            next_agent_id: 0,
            agents,
            world,
            generation: 0,
            config: config.clone(),
            rng: RNG::new(config.random_seed.unwrap_or(0)),
        }
    }

    pub fn populate(&mut self) {
        for _ in 0..self.config.initial_agent_count {
            let x = self.rng.choose_range(0..self.config.grid_width);
            let y = self.rng.choose_range(0..self.config.grid_height);
            let position = Coordinate::new(x, y);
            let id = self.new_agent_id();

            let agent = Agent::new(id, position);
            self.agents.push(agent);
            self.world.place_agent(id, position);
        }
    }

    pub fn spread_resources(&mut self) {
        for _ in 0..self.config.initial_resource_count {
            let x = self.rng.choose_range(0..self.config.grid_width);
            let y = self.rng.choose_range(0..self.config.grid_height);
            let position = Coordinate::new(x, y);

            let resource = Resource::new(ResourceType::FoodSource);
            self.world.place_resource(position, resource);
        }
    }

    pub fn new_agent_id(&mut self) -> u32 {
        let id = self.next_agent_id;
        self.next_agent_id += 1;
        id
    }

    pub fn get_agents(&self) -> &Vec<Agent> {
        &self.agents
    }

    pub fn get_world(&self) -> &Grid {
        &self.world
    }

    pub fn print_world_dim(&self) {
        self.world.print_dim();
    }

    pub fn print_world(&self) {
        self.world.print();
    }
}
