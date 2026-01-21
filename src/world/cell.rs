use crate::display::tile::Tile;
use crate::world::resource::Resource;

pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub resources: Option<Vec<Resource>>,
    pub inhabitant_ids: Vec<u32>,
    pub territory_owner_id: Option<u32>,
}

impl Cell {
    pub fn to_tile(&self) -> Tile {
        // Placeholder logic for converting a Cell to a Tile
        Tile::new(
            self.x,
            self.y,
            [0, 255, 0, 255],
        )
    }
}
