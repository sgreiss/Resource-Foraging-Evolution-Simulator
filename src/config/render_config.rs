pub struct RenderConfig {
    render_width: u32,
    render_height: u32,
    tile_size: usize,
}

impl RenderConfig {
    pub fn default() -> Self {
        RenderConfig {
            render_width: 800,
            render_height: 600,
            tile_size: 10,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.render_width
    }

    pub fn get_height(&self) -> u32 {
        self.render_height
    }

    pub fn get_tile_size(&self) -> usize {
        self.tile_size
    }
}
