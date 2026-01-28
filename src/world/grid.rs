use crate::Coordinate;
use crate::agent::agent::Agent;
use crate::display::tile::Tile;
use crate::ids::*;
use crate::print_colored;
use crate::world::{cell::Cell, resource::Resource};

#[derive(Clone, Debug)]
pub struct Grid {
    width: u32,
    height: u32,
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        let mut grid = Vec::new();
        let mut id_manager = IdManager::new();
        for x in 0..width {
            let mut column = Vec::new();
            for y in 0..height {
                let cell_id = id_manager.next_id::<Cell>();
                column.push(Cell::new(
                    cell_id,
                    Coordinate::new(x, y),
                    Vec::new(),
                    Vec::new(),
                    None,
                ));
            }
            grid.push(column);
        }
        Grid {
            width,
            height,
            grid,
        }
    }

    pub fn place_agent(&mut self, agent_id: Id<Agent>, coordinate: Coordinate) {
        let (x, y) = coordinate.as_u32();
        self.grid[x as usize][y as usize].add_inhabitant(agent_id);
    }

    pub fn place_resource(&mut self, coordinate: Coordinate, resource: Resource) {
        let (x, y) = coordinate.as_u32();
        self.grid[x as usize][y as usize].add_resource(resource);
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Tile {
        self.grid[x as usize][y as usize].to_tile()
    }

    pub fn cell_at(&self, x: u32, y: u32) -> &Cell {
        &self.grid[x as usize][y as usize]
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_grid(&self) -> &Vec<Vec<Cell>> {
        &self.grid
    }

    pub fn print_dim(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = self.grid[x as usize][y as usize].position;
                print!("{}", position);
            }
            println!();
        }
    }
    pub fn print(&self) {
        println!("┌{}┐", "─".repeat((3 * self.width + 1) as usize));
        for y in 0..self.height {
            print!("│ ");
            for x in 0..self.width {
                if self.grid[x as usize][y as usize].has_resources() {
                    print_colored("R", crossterm::style::Color::Green);
                } else {
                    print!(".");
                }
                if self.grid[x as usize][y as usize].has_inhabitant() {
                    print_colored("A", crossterm::style::Color::Red);
                } else {
                    print!(".");
                }
                print!(" ");
            }
            println!("│");
        }
        println!("└{}┘", "─".repeat((3 * self.width + 1) as usize));
    }
}
