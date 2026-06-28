use chrono_sub::io;
use std::process;

fn main() {
    if let Err(e) = io::start() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
