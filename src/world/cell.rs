use crate::display::tile::Tile;
use crate::world::resource::Resource;

pub struct Cell {
    pub position: (u32, u32),
    resources: Option<Vec<Resource>>,
    inhabitant_ids: Vec<u32>,
    territory_owner_id: Option<u32>,
}

impl Cell {
    pub fn new(
        position: (u32, u32),
        resources: Option<Vec<Resource>>,
        inhabitant_ids: Vec<u32>,
        territory_owner_id: Option<u32>,
    ) -> Self {
        Cell {
            position,
            resources,
            inhabitant_ids,
            territory_owner_id,
        }
    }

    pub fn add_inhabitant(&mut self, agent_id: u32) {
        self.inhabitant_ids.push(agent_id);
    }

    pub fn to_tile(&self) -> Tile {
        // Placeholder logic for converting a Cell to a Tile
        Tile::new(self.position.0, self.position.1, [0, 255, 0, 255])
    }

    pub fn has_resources(&self) -> bool {
        self.resources.is_some()
    }

    pub fn has_inhabitant(&self) -> bool {
        !self.inhabitant_ids.is_empty()
    }

    pub fn is_territory(&self) -> bool {
        self.territory_owner_id.is_some()
    }
}
