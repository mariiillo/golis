mod entities;
mod use_cases;
mod ui;

use crate::use_cases::calculate_next_generation;
use crate::ui::print_in_console;

fn main() {
    let mut universe: entities::Universe = entities::Universe::new();

    loop {
        print_in_console(&universe);
        calculate_next_generation(&mut universe);
    }
}