#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum Cell {
    Alive(Position),
    Dead(Position),
}

#[derive(Debug)]
struct Game {
    world: Vec<Cell>,
}

impl Game {
    pub fn new() -> Game {
        let mut cells: Vec<Cell> = Vec::new();
        cells.push(Cell::Alive(Position { x: 0, y: 0 }));
        cells.push(Cell::Dead(Position { x: 0, y: 1 }));

        Game {
            world: cells,
        }
    }
}

fn main() {
    let mut game: Game = Game::new();

    println!("World is {:?}", game);

    game.world.push(Cell::Dead(Position { x: 0, y: 2 }));

    println!("World is {:?}", game);
}
