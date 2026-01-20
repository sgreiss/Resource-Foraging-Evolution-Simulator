pub struct SimConfig {
    pub grid_width: u32,
    pub grid_height: u32,
    initial_agent_count: u32,
    resource_spawn_rate: f32,
    max_simulation_steps: u32,
    mutation_rate: f32,
    random_seed: Option<u64>,
}

impl Default for SimConfig {
    fn default() -> Self {
        SimConfig {
            grid_width: 10,
            grid_height: 10,
            initial_agent_count: 5,
            resource_spawn_rate: 0.1,
            max_simulation_steps: 1000,
            mutation_rate: 0.01,
            random_seed: Some(42),
        }
    }
}
