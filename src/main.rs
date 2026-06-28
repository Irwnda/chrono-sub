use chrono_sub::start;
use std::process;

fn main() {
    if let Err(e) = start() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
