use crate::Coordinate;
use crate::agent::agent::Agent;
use crate::display::tile::Tile;
use crate::utils::ids::Id;
use crate::world::resource::Resource;

#[derive(Clone, Debug)]
pub struct Cell {
    pub id: Id<Cell>,
    pub position: Coordinate,
    resources: Vec<Resource>,
    inhabitant_ids: Vec<Id<Agent>>,
    territory_owner_id: Option<Id<Agent>>,
}

impl Cell {
    pub fn new(
        id: Id<Cell>,
        position: Coordinate,
        resources: Vec<Resource>,
        inhabitant_ids: Vec<Id<Agent>>,
        territory_owner_id: Option<Id<Agent>>,
    ) -> Self {
        Cell {
            id,
            position,
            resources,
            inhabitant_ids,
            territory_owner_id,
        }
    }

    pub fn add_inhabitant(&mut self, agent_id: Id<Agent>) {
        self.inhabitant_ids.push(agent_id);
    }

    pub fn add_resource(&mut self, resource: Resource) {
        self.resources.push(resource)
    }

    pub fn to_tile(&self) -> Tile {
        // Placeholder logic for converting a Cell to a Tile
        Tile::new(self.id.convert::<Tile>(), self.position, [0, 255, 0, 255])
    }

    pub fn has_resources(&self) -> bool {
        !self.resources.is_empty()
    }

    pub fn has_inhabitant(&self) -> bool {
        !self.inhabitant_ids.is_empty()
    }

    pub fn is_territory(&self) -> bool {
        self.territory_owner_id.is_some()
    }
}
