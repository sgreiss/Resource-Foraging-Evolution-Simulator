use crate::display::tile::Tile;
use crate::config::RenderConfig;
use crate::world;

pub struct Renderer {
    width: usize,
    height: usize,
    frame: Vec<u8>,
    config: RenderConfig,
}

impl Renderer {
    pub fn new(config: RenderConfig) -> Self {
        let frame = vec![0; (config.render_width * config.render_height * 4) as usize];
        Self {
            width: config.render_width as usize,
            height: config.render_height as usize,
            frame,
            config,
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: [u8; 4]) {
        let index = (y * self.width + x) * 4;
        self.frame[index..index + 4].copy_from_slice(&color);
    }

    pub fn draw_tile(&mut self, x: usize, y: usize, tile: &Tile) {
        let start_x = x * self.config.tile_size;
        let start_y = y * self.config.tile_size;
        for dy in 0..self.config.tile_size {
            for dx in 0..self.config.tile_size {
                let pixel_x = start_x + dx;
                let pixel_y = start_y + dy;
                if pixel_x < self.width && pixel_y < self.height {
                    self.set_pixel(pixel_x, pixel_y, tile.color);
                }
            }
        }
    }

    pub fn render(&mut self, world: &world::grid::Grid) {
        for y in 0..world.height {
            for x in 0..world.width {
                let tile = world.get_tile(x, y);
                self.draw_tile(x as usize, y as usize, &tile);
            }
        }
    }

    pub fn get_frame(&self) -> &[u8] {
        &self.frame
    }

    pub fn clear(&mut self) {
        self.frame.fill(0);
    }
}
