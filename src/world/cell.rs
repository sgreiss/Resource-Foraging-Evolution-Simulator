struct Cell {
    x: u32,
    y: u32,
    resource_amount: u32,
    inhabitant_ids: Vec<u32>,
    territory_owner_id: Option<u32>,
}