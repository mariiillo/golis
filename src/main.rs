mod entities;
mod use_cases;

use std::{thread, time};
use crate::use_cases::calculate_next_generation;

fn main() {
    let mut universe: entities::Universe = entities::Universe::new();

    println!("{}", universe);

    loop {
        thread::sleep(time::Duration::from_millis(300));
        print!("{}[2J", 27 as char);
        calculate_next_generation(&mut universe);
        println!("{}", universe);
    }
}