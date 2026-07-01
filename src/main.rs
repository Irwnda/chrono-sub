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

            if let Err(err) = subtitle::process(file) {
                eprintln!("{}", style(format!("❌ {}", err)).with(Color::Red).bold());
                process::exit(1);
            }
        }
        Err(err) => {
            println!("Application error: {}", style(format!("❌ {}", err)).with(Color::Red).bold());
            process::exit(1);
        }
    }
}
