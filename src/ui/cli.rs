use std::{thread, time};
use std::fmt;

pub fn print(universe: &dyn fmt::Display) {
    thread::sleep(time::Duration::from_millis(200));
    println!("{}", universe);
    print!("{}[2J", 27 as char);
}