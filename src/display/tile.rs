pub struct Tile {
    x: u32,
    y: u32,
    color: [u8; 4],
}

impl Tile {
    pub fn new(x: u32, y: u32, color: [u8; 4]) -> Self {
        Tile { x, y, color }
    }

    pub fn get_position(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    pub fn get_color(&self) -> [u8; 4] {
        self.color
    }
}