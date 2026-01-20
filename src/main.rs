use resource_foraging_evolution_simulator::*;

fn main() {
    let sim_config = SimConfig::default();
    let render_config = RenderConfig::default();

    let mut engine = Engine::new(sim_config);
    let mut renderer = Renderer::new(render_config);

}