use resource_foraging_evolution_simulator::*;

fn main() {
    let sim_config = SimConfig::default();
    let mut engine = Engine::new(sim_config);

    clear_screen();
    engine.print_grid();

    print!("\nPopulating world...\n");

    engine.populate();
    engine.print_grid();

    print!("\nSpreading resources...\n");
    engine.spread_resources();
    engine.print_grid();
}
