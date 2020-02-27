use rand::Rng;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Cell {
    Alive = 1,
    Dead = 0,
}

#[derive(Debug)]
struct World {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl World {
    pub fn new() -> World {
        let mut cells: Vec<Cell> = Vec::new();
        let mut random_generator = rand::thread_rng();

        for _n in 0..9 {
            let random = random_generator.gen_range(0.0, 1.0);
            if random > 0.5 {
                cells.push(Cell::Alive);
            } else {
                cells.push(Cell::Dead);
            }
        }

        World {
            width: 3,
            height: 3,
            cells,
        }
    }

    pub fn print(&self) {
        for row in 0..3 {
            for column in 0..3 {
                print!("{:?} ", self.cell_at(row, column) as u8);
            }
            println!("\n");
        }
    }

    fn cell_at(&self, row: u32, column: u32) -> Cell {
        let index = (row * self.width + column) as usize;

        self.cells[index]
    }

    fn cells_alive_around(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let cell = self.cell_at(neighbor_row, neighbor_col);
                count += cell as u8;
            }
        }
        count
    }
}

fn main() {
    let world: World = World::new();
    world.print();
    println!("{}", world.cells_alive_around(2, 2));
}
