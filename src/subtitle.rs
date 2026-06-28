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

    let time_adjustment = adjustment_duration();

    println!("Adjusting by {}...", time_adjustment);

    match direction {
        0 => {},
        1 => {},
        _ => {}
    }
}

fn adjustment_duration() -> String {
    let time_regex = Regex::new(r"^\d{2}:\d{2}:\d{2}\.\d{2,3}$").unwrap();

    Text::new("Enter the adjustment duration:")
        .with_placeholder("hh:mm:ss.ms (e.g., 00:00:01.500 for 1.5 seconds)")
        .with_help_message("Format must be hours:minutes:seconds.milliseconds")
        .with_validator(move |input: &str| {
            if time_regex.is_match(input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Invalid format! Please use hh:mm:ss.ms".into()))
            }
        })
        .prompt()
        .unwrap()
}
