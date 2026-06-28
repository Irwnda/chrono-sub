use std::path::PathBuf;
use inquire::{Select, Text, validator::Validation};
use regex::Regex;

enum Direction {
    Forward,
    Backward,
}

pub fn process_file(file: PathBuf) {
    let direction_options = vec![
        "Slower (Delay subtitles / Shift Forward / +Time)",
        "Faster (Speed up subtitles / Shift Backward / -Time)",
    ];

    let direction = Select::new("Do you want to make the subtitles faster or slower?", direction_options)
        .raw_prompt()
        .unwrap()
        .index;

    match direction {
        0 => {},
        1 => {},
        _ => {}
    }
}
