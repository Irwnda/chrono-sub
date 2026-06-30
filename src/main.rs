use chrono_sub::{args, io, subtitle};
use crossterm::style::{Color, Stylize, style};
use std::process;

fn main() {
    args::parse();

    match io::start() {
        Ok(file) => {
            println!(
                "Selected file: {}",
                style(file.display()).with(Color::Green).bold()
            );

            let _ = subtitle::process(file);
        }
        Err(err) => {
            println!("Application error: {}", err);
            process::exit(1);
        }
    }
}
