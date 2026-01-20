use crate::agent::genome::Genome;
use crate::world::cell::Cell;

pub struct Agent {
    id: u32,
    name: String,
    genome: Genome,
    location: Cell,
    territory: Vec<Cell>,
    energy: u32,
    age: u32,
}
