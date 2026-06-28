use chrono_sub::{io, subtitle};
use crossterm::style::{style, Color, Stylize};
use std::process;

fn main() {
    match io::start() {
        Ok(file) => {
            println!("Selected file: {}", style(file.display()).with(Color::Green).bold());

            subtitle::process_file(file);
        },
        Err(err) => {
            println!("Application error: {}", err);
            process::exit(1);
        }
    }
}
