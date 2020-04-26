pub struct State {
    pub grid: Vec<Vec<bool>>,
    pub x: usize,
    pub y: usize,
    pub generation: u32,
}

impl State {
    pub fn new(x: usize, y: usize) -> State {
        let mut grid = Vec::<Vec<bool>>::with_capacity(x);
        for i in 0..x {
            let v = i == 2;
            grid.push((0..y).map(|_| v).collect());
        };
        return State {
            grid,
            x,
            y,
            generation: 0,
        };
    }

    pub fn next(&mut self) {
        let new_grid: Vec<Vec<bool>> = (0..self.x).map(|x| {
            (0..self.y).map(|y| {
                let mut neighbors: u8 = 0;
                // add x - 1 neighbors (left neighbors)
                if x > 0 {
                    neighbors += if y > 0 && self.grid[x - 1][y - 1] { 1 } else { 0 };
                    neighbors += if self.grid[x - 1][y] { 1 } else { 0 };
                    neighbors += if y < (self.y - 1) && self.grid[x - 1][y + 1] { 1 } else { 0 };
                }

                // add x neighbors (top and bottom)
                neighbors += if y > 0 && self.grid[x][y - 1] { 1 } else { 0 };
                neighbors += if y < (self.y - 1) && self.grid[x][y + 1] { 1 } else { 0 };

                // add x + 1 neighbors (right neighbors)
                if x < self.x - 1 {
                    neighbors += if y > 0 && self.grid[x + 1][y - 1] { 1 } else { 0 };
                    neighbors += if self.grid[x + 1][y] { 1 } else { 0 };
                    neighbors += if y < (self.y - 1) && self.grid[x + 1][y + 1] { 1 } else { 0 };
                }

                let is_alive =
                    // Alive cells stay alive if 2 or 3 neighbors
                    (self.grid[x][y] && (neighbors == 2 || neighbors == 3)) ||
                        // Dead cells become alive if 3 neighbors
                        (!self.grid[x][y] && neighbors == 3);
                return is_alive;
            }).collect()
        }).collect();

        self.grid = new_grid;
        self.generation += 1;
    }
}
