use crate::utils::Coordinate;
use crate::utils::ids::Id;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    id: Id<Tile>,
    position: Coordinate,
    color: [u8; 4],
}

impl Tile {
    pub fn new(id: Id<Tile>, position: Coordinate, color: [u8; 4]) -> Self {
        Tile { id, position, color }
    }

    pub fn id(&self) -> Id<Tile> {
        self.id
    }

    pub fn position(&self) -> Coordinate {
        self.position
    }

    pub fn color(&self) -> [u8; 4] {
        self.color
    }
}
