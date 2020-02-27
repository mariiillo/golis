use rand::Rng;

#[derive(Debug)]
enum Cell {
    Alive,
    Dead,
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

        for _n in 0..101 {
            let random = random_generator.gen_range(0.0, 1.0);
            if random > 0.5 {
                cells.push(Cell::Alive);
            } else {
                cells.push(Cell::Dead);
            }
        }

        World {
            width: 10,
            height: 10,
            cells,
        }
    }

    pub fn print(&self) {
        for _row in 0..11 {
            for column in 0..11 {
                println!("{:?}", self.cells[column]);
            }
            println!("\n");
        }
    }
}

fn main() {
    let mut world: World = World::new();

    world.cells.push(Cell::Dead);

    world.print();
}
