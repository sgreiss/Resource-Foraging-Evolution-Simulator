use crate::Coordinate;
use crate::agent::genome::Genome;
use crate::utils::ids::Id;

#[derive(Clone, Debug)]
pub struct Agent {
    pub id: Id<Agent>,
    pub name: String,
    genome: Genome,
    location: Coordinate,
    territory: Vec<Coordinate>,
    energy: u32,
    age: u32,
}

impl Agent {
    pub fn new(id: Id<Agent>, position: Coordinate) -> Self {
        Agent {
            id: id.clone(),
            name: format!("Agent_{}", id.raw()),
            genome: Genome::random(),
            location: position,
            territory: Vec::new(),
            energy: 100,
            age: 0,
        }
    }
}
