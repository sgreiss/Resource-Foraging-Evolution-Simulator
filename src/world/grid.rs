use crate::display::tile::Tile;
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
                column.push(Cell {
                    x,
                    y,
                    resources: None,
                    inhabitant_ids: Vec::new(),
                    territory_owner_id: None,
                });
            }
            grid.push(column);
        }
        Grid {
            width,
            height,
            grid,
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "({},{}) ",
                    self.grid[x as usize][y as usize].x, self.grid[x as usize][y as usize].y
                );
            }
            println!();
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Tile {
        self.grid[x as usize][y as usize].to_tile()
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
}
