mod entities;
mod use_cases;
mod ui;

fn main() {
    let mut universe: entities::Universe = entities::Universe::new();

    loop {
        ui::cli::print(&universe);
        use_cases::calculate_next_generation(&mut universe);
    }
}