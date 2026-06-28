use chrono_sub::{io, subtitle};
use std::process;

fn main() {
    match io::start() {
        Ok(file) => {
            subtitle::process_file(file);
        },
        Err(err) => {
            println!("Application error: {}", err);
            process::exit(1);
        }
    }
}
