#[derive(Clone, Copy, Debug)]
pub struct RenderConfig {
    pub render_width: u32,
    pub render_height: u32,
    pub tile_size: usize,
}

impl RenderConfig {
    pub fn default() -> Self {
        RenderConfig {
            render_width: 800,
            render_height: 600,
            tile_size: 10,
        }
    }
}
