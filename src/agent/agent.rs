use crate::agent::genome::Genome;

pub struct Agent {
    pub id: u32,
    pub name: String,
    genome: Genome,
    location: (u32, u32),
    territory: Vec<(u32, u32)>,
    energy: u32,
    age: u32,
}

impl Agent {
    pub fn new(id: u32, position: (u32, u32)) -> Self {
        Agent {
            id,
            name: format!("Agent_{}", id),
            genome: Genome::random(),
            location: position,
            territory: Vec::new(),
            energy: 100,
            age: 0,
        }
    }
}
