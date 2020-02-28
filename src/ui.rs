use std::fmt;
use std::{thread, time};

pub fn print_in_console(universe: &dyn fmt::Display) {
    thread::sleep(time::Duration::from_millis(300));
    println!("{}", universe);
    print!("{}[2J", 27 as char);
}