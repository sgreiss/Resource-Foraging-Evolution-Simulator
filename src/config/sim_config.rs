#[derive(Clone, Copy, Debug)]
pub struct SimConfig {
    pub grid_width: u32,
    pub grid_height: u32,
    pub initial_agent_count: u32,
    pub initial_resource_count: u32,
    pub resource_spawn_rate: f32,
    pub max_simulation_steps: u32,
    pub mutation_rate: f32,
    pub random_seed: Option<u64>,
}

impl SimConfig {
    pub fn default() -> Self {
        SimConfig {
            grid_width: 50,
            grid_height: 50,
            initial_agent_count: 15,
            initial_resource_count: 25,
            resource_spawn_rate: 0.1,
            max_simulation_steps: 1000,
            mutation_rate: 0.01,
            random_seed: Some(42),
        }
    }
}
