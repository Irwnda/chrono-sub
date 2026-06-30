use crate::subtitle::{time::Direction, transform};
use inquire::validator::Validation;
use inquire::{Select, Text};
use std::path::{Path, PathBuf};

pub(crate) fn prompt_direction() -> Direction {
    let direction_options = vec![
        "Slower (Delay subtitles / Shift Forward / +Time)",
        "Faster (Speed up subtitles / Shift Backward / -Time)",
    ];

    let direction = Select::new(
        "Do you want to make the subtitles faster or slower?",
        direction_options,
    )
    .raw_prompt()
    .unwrap()
    .index;

    match direction {
        0 => Direction::Forward,
        1 => Direction::Backward,
        _ => Direction::Forward,
    }
}

pub(crate) fn adjustment_duration() -> String {
    Text::new("Enter the adjustment duration:")
        .with_placeholder("hh:mm:ss,ms (e.g., 00:00:01,500 for 1.5 seconds)")
        .with_help_message(
            "Format must be hours:minutes:seconds,milliseconds (or you can use . for milliseconds)",
        )
        .with_validator(move |input: &str| {
            if transform::validate_time(input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "Invalid format! Please use hh:mm:ss.ms".into(),
                ))
            }
        })
        .prompt()
        .unwrap()
}

pub(crate) fn prompt_naming(file: &Path) -> PathBuf {
    let naming_options = vec![
        "Add a suffix (e.g., filename_adjusted.srt)",
        "Add a prefix (e.g., adjusted_filename.srt)",
        "Replace the original file",
        "Give it a completely custom name",
    ];
    let naming_choice = Select::new(
        "How would you like to name the output file?",
        naming_options,
    )
    .raw_prompt()
    .unwrap()
    .index;

    let file_stem = file.file_stem().unwrap().to_str().unwrap();
    let extension = file.extension().unwrap().to_str().unwrap();
    let output_name = match naming_choice {
        0 => {
            let suffix = Text::new("Enter suffix:")
                .with_default("_adjusted")
                .prompt()
                .unwrap();
            format!("{}{}.{}", file_stem, suffix, extension)
        }
        1 => {
            let prefix = Text::new("Enter prefix:")
                .with_default("adjusted_")
                .prompt()
                .unwrap();
            format!("{}{}.{}", prefix, file_stem, extension)
        }
        2 => file_stem.to_string(),
        _ => Text::new("Enter completely new filename (with extension):")
            .with_default(&format!("{}_new.{}", file_stem, extension))
            .prompt()
            .unwrap(),
    };

    let directory = file.parent().unwrap_or_else(|| Path::new("."));

    directory.join(output_name)
}
