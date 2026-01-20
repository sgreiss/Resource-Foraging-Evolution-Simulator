use crate::world::resource::Resource;

pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub resources: Option<Vec<Resource>>,
    pub inhabitant_ids: Vec<u32>,
    pub territory_owner_id: Option<u32>,
}