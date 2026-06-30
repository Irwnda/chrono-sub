mod encoding;
mod prompt;
mod time;
mod transform;

use crossterm::style::{Color, Stylize, style};
use std::path::PathBuf;

pub fn process(file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let direction = prompt::prompt_direction();
    let time_adjustment = prompt::adjustment_duration();

    let sub_time = match time::SubTime::from_str(&time_adjustment) {
        Some(st) => st,
        None => {
            println!(
                "{}",
                style("❌ Invalid time format entered. Exiting.")
                    .with(Color::Red)
                    .bold()
            );
            return Err("Invalid time format".into());
        }
    };

    let separator = match transform::separator(&file) {
        Some(s) => s,
        None => {
            println!(
                "{}",
                style("❌ Invalid subtitle extension. Exiting.")
                    .with(Color::Red)
                    .bold()
            );
            return Err("Invalid subtitle extension".into());
        }
    };
    let (sub_content, enc) = encoding::read_subtitle_file(&file)?;
    let new_content =
        match transform::transform_subtitle(&sub_content, &sub_time, &direction, separator) {
            Ok(result) => result,
            Err(e) => {
                println!("{}", style(&e).with(Color::Red).bold());
                return Err(e.into());
            }
        };

    encoding::save_file(&file, &new_content, enc);
    Ok(())
}
