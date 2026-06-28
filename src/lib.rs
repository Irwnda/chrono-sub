use inquire::{Select, Text};
use std::{env, error::Error};
use std::path::PathBuf;

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
    let current_dir = env::current_dir()?;
    let target_path = match option {
        0 => current_dir,
        1 => {
            let path_str = Text::new("Enter the folder path:")
                .with_default(".")
                .prompt()
                .unwrap_or_else(|_| String::from("."));
            PathBuf::from(path_str)
        },
        2 => browse_folders(current_dir),
        _ => return Err("Invalid option".into())
    };

    println!("Target path: {:?}", target_path);

    Ok(())
}

fn browse_folders(_: PathBuf) -> PathBuf {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proceed_option_0_returns_ok() {
        let result = proceed(&0);
        assert!(result.is_ok(), "Expected Ok for option 0 (current directory)");
    }

    #[test]
    fn proceed_invalid_option_returns_err() {
        let result = proceed(&99);
        assert!(result.is_err(), "Expected Err for an invalid option index");
    }
}