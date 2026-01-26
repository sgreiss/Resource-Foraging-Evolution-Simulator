use crate::display::tile::Tile;
use crate::utils::print_colored;
use crate::world::cell::Cell;

pub struct Grid {
    width: u32,
    height: u32,
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        let mut grid = Vec::new();
        for x in 0..width {
            let mut column = Vec::new();
            for y in 0..height {
                column.push(Cell::new((x, y), None, Vec::new(), None));
            }
            grid.push(column);
        }
        Grid {
            width,
            height,
            grid,
        }
    }

    pub fn place_agent(&mut self, agent_id: u32, x: u32, y: u32) {
        self.grid[x as usize][y as usize].add_inhabitant(agent_id);
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
                print!(
                    "({}, {}) ",
                    self.grid[x as usize][y as usize].position.0,
                    self.grid[x as usize][y as usize].position.1
                );
            }
            println!();
        }
    }
    pub fn print(&self) {
        println!("┌{}┐", "──".repeat(self.width as usize));
        for y in 0..self.height {
            print!("│");
            for x in 0..self.width {
                if self.grid[x as usize][y as usize].has_resources() {
                    print_colored("R ", crossterm::style::Color::Green);
                } else if self.grid[x as usize][y as usize].has_inhabitant() {
                    print_colored("A ", crossterm::style::Color::Blue);
                } else {
                    print_colored(". ", crossterm::style::Color::DarkGrey);
                }
            }
            println!("│");
        }
        println!("└{}┘", "──".repeat(self.width as usize));
    }
}
