use resource_foraging_evolution_simulator::*;

fn main() {
    let sim_config = SimConfig::default();

    let engine = Engine::new(sim_config);

    clear_screen();
    engine.print_grid();
}