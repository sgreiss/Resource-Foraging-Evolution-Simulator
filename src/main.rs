use resource_foraging_evolution_simulator::*;

fn main() {
    let config: Config = Config::default();
    let engine: Engine = Engine::new(config);
    
    clear_screen();
    engine.print_grid();
}
