use crate::subtitle::{time::Direction, transform};
use inquire::validator::Validation;
use inquire::{Select, Text};
use std::path::{Path, PathBuf};

pub(crate) fn prompt_direction() -> Result<Direction, String> {
    let direction_options = vec![
        "Slower (Delay subtitles / Shift Forward / +Time)",
        "Faster (Speed up subtitles / Shift Backward / -Time)",
    ];

    let direction = match Select::new(
        "Do you want to make the subtitles faster or slower?",
        direction_options,
    )
    .raw_prompt()
    {
        Ok(choice) => choice.index,
        Err(_) => return Err("Failed to get subtitle edit direction".into()),
    };

    match direction {
        0 => Ok(Direction::Forward),
        1 => Ok(Direction::Backward),
        _ => Ok(Direction::Forward),
    }
}

pub(crate) fn adjustment_duration() -> Result<String, String> {
    match Text::new("Enter the adjustment duration:")
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
    {
        Ok(time) => Ok(time),
        Err(_) => Err("Failed to enter time".into()),
    }
}

pub(crate) fn prompt_naming(file: &Path) -> Result<PathBuf, String> {
    let naming_options = vec![
        "Add a suffix (e.g., filename_adjusted.srt)",
        "Add a prefix (e.g., adjusted_filename.srt)",
        "Replace the original file",
        "Give it a completely custom name",
    ];
    let naming_choice = match Select::new(
        "How would you like to name the output file?",
        naming_options,
    )
    .raw_prompt()
    {
        Ok(choice) => choice.index,
        Err(_) => return Err("Failed to get output name".into()),
    };

    let file_stem = file
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("File name is not valid UTF-8")?;
    let extension = file
        .extension()
        .and_then(|e| e.to_str())
        .ok_or("File extension is not valid UTF-8")?;
    let output_name = match naming_choice {
        0 => {
            let suffix = match Text::new("Enter suffix:")
                .with_default("_adjusted")
                .prompt()
            {
                Ok(suffix) => suffix,
                Err(_) => return Err("Failed to get suffix".into()),
            };
            format!("{}{}.{}", file_stem, suffix, extension)
        }
        1 => {
            let prefix = match Text::new("Enter prefix:")
                .with_default("adjusted_")
                .prompt()
            {
                Ok(prefix) => prefix,
                Err(_) => return Err("Failed to get prefix".into()),
            };
            format!("{}{}.{}", prefix, file_stem, extension)
        }
        2 => file_stem.to_string(),
        _ => match Text::new("Enter completely new filename (with extension):")
            .with_default(&format!("{}_new.{}", file_stem, extension))
            .prompt()
        {
            Ok(name) => name,
            Err(_) => return Err("Failed to get new filename".into()),
        },
    };

    let directory = file.parent().unwrap_or_else(|| Path::new("."));

    Ok(directory.join(output_name))
}
