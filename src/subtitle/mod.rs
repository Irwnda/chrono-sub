mod encoding;
mod prompt;
mod time;
mod transform;

use std::path::PathBuf;

pub fn process(file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let direction = prompt::prompt_direction()?;
    let time_adjustment = prompt::adjustment_duration()?;
    let sub_time = time::SubTime::from_str(&time_adjustment).ok_or("Invalid time format")?;
    let separator = transform::separator(&file).ok_or("Invalid subtitle extension")?;

    let (sub_content, enc) = encoding::read_subtitle_file(&file)?;
    let new_content =
        transform::transform_subtitle(&sub_content, &sub_time, &direction, separator)?;

    encoding::save_file(&file, &new_content, enc)?;
    Ok(())
}
