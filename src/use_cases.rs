use crate::entities::{Universe, Cell};

pub fn calculate_next_generation(universe: &mut Universe) {
    let mut next = universe.cells.clone();

    for row in 0..universe.height {
        for col in 0..universe.width {
            let idx = universe.get_index(row, col);
            let cell = universe.cells[idx];
            let live_neighbors = count_neighbors_alive(universe, row, col);

            let next_cell = match (cell, live_neighbors) {
                (Cell::Alive, x) if x < 2 => Cell::Dead,
                (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                (Cell::Alive, x) if x > 3 => Cell::Dead,
                (Cell::Dead, 3) => Cell::Alive,
                (otherwise, _) => otherwise,
            };
            next[idx] = next_cell;
        }
    }

    universe.cells = next;
}

fn count_neighbors_alive(universe: &Universe, row: u32, column: u32) -> u8 {
    let mut count = 0;
    for delta_row in [universe.height - 1, 0, 1].iter().cloned() {
        for delta_col in [universe.width - 1, 0, 1].iter().cloned() {
            if delta_row == 0 && delta_col == 0 {
                continue;
            }

            let neighbor_row = (row + delta_row) % universe.height;
            let neighbor_col = (column + delta_col) % universe.width;
            let idx = universe.get_index(neighbor_row, neighbor_col);
            count += universe.cells[idx] as u8;
        }
    }
    count
}