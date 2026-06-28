use inquire::{Select, Text};
use std::error::Error;

pub fn start() -> Result<(), Box<dyn Error>>{
    let options = vec![
        "Use current directory",
        "Enter a path",
        "Browse for a file"
    ];

    let selected = Select::new("How would you like to select the target folder?", options).raw_prompt().unwrap().index;

    proceed(&selected)
}

fn proceed(option: &usize) -> Result<(), Box<dyn Error>>{
    match option {
        0 => println!("Using current directory"),
        1 => println!("Entering a path"),
        2 => println!("Browsing for a file"),
        _ => println!("Invalid option"),
    }
    Ok(())
}
