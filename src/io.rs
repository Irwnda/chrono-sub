use inquire::{Select, Text};
use std::path::PathBuf;
use std::{env, error::Error, fs};

pub fn start() -> Result<PathBuf, Box<dyn Error>> {
    let options = vec!["Use current directory", "Enter a path", "Browse for a file"];

    let selected = Select::new("How would you like to select the target folder?", options)
        .raw_prompt()
        .unwrap()
        .index;

    proceed(&selected)
}

fn proceed(option: &usize) -> Result<PathBuf, Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let mut target_path = match option {
        0 => current_dir,
        1 => {
            let path_str = Text::new("Enter the path:")
                .with_default(".")
                .prompt()
                .unwrap_or_else(|_| String::from("."));
            PathBuf::from(path_str.trim())
        }
        2 => browse_path(current_dir),
        _ => return Err("Invalid option".into()),
    };

    if !target_path.is_file() {
        target_path = browse_path(target_path);
    }

    Ok(target_path)
}

fn browse_path(mut current_dir: PathBuf) -> PathBuf {
    loop {
        let choices = choice_from_dir(&current_dir);
        let selected = Select::new("Select folder or a file:", choices).prompt();

        match selected {
            Ok(choice) if choice.starts_with("..") => {
                if let Some(parent) = current_dir.parent() {
                    current_dir = parent.to_path_buf();
                }
            }
            Ok(choice) if choice.ends_with("/") => {
                let clean_name = choice.trim_end_matches('/');
                current_dir.push(clean_name);
            }
            Ok(choice) => {
                return current_dir.join(choice);
            }
            _ => {
                println!("Browsing cancelled. Defaulting to current folder.");
                break;
            }
        }
    }

    current_dir
}

fn choice_from_dir(dir: &PathBuf) -> Vec<String> {
    let mut choices = vec![String::from(".. (Go Up)")];

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        choices.push(format!("{}/", name))
                    }
                }

                if file_type.is_file() {
                    if let Some(ext) = entry.path().extension() {
                        if ["srt", "vtt"].contains(&ext.to_str().unwrap().to_lowercase().as_str()) {
                            choices.push(format!("{}", entry.file_name().to_str().unwrap()))
                        }
                    }
                }
            }
        }
    }

    choices
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self as stdfs, File};
    use std::io::Write;

    // ── proceed ──────────────────────────────────────────────────────────────

    #[test]
    fn proceed_invalid_option_returns_err() {
        let result = proceed(&99);
        assert!(result.is_err(), "Expected Err for an invalid option index");
    }

    // ── choice_from_dir ───────────────────────────────────────────────────────

    fn make_temp_dir(name: &str) -> PathBuf {
        let dir = env::temp_dir().join(name);
        stdfs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn choice_from_dir_always_has_go_up_first() {
        let dir = make_temp_dir("chronosub_test_go_up");
        let choices = choice_from_dir(&dir);
        assert_eq!(choices[0], ".. (Go Up)");
    }

    #[test]
    fn choice_from_dir_includes_directories_with_slash() {
        let root = make_temp_dir("chronosub_test_dirs");
        stdfs::create_dir_all(root.join("subdir")).unwrap();

        let choices = choice_from_dir(&root);
        assert!(
            choices.contains(&String::from("subdir/")),
            "Expected 'subdir/' in choices, got: {:?}",
            choices
        );
    }

    #[test]
    fn choice_from_dir_includes_srt_files() {
        let dir = make_temp_dir("chronosub_test_srt");
        File::create(dir.join("subtitle.srt"))
            .unwrap()
            .write_all(b"")
            .unwrap();

        let choices = choice_from_dir(&dir);
        assert!(
            choices.contains(&String::from("subtitle.srt")),
            "Expected 'subtitle.srt' in choices, got: {:?}",
            choices
        );
    }

    #[test]
    fn choice_from_dir_includes_vtt_files() {
        let dir = make_temp_dir("chronosub_test_vtt");
        File::create(dir.join("subtitle.vtt"))
            .unwrap()
            .write_all(b"")
            .unwrap();

        let choices = choice_from_dir(&dir);
        assert!(
            choices.contains(&String::from("subtitle.vtt")),
            "Expected 'subtitle.vtt' in choices, got: {:?}",
            choices
        );
    }

    #[test]
    fn choice_from_dir_excludes_unsupported_extensions() {
        let dir = make_temp_dir("chronosub_test_exclude");
        File::create(dir.join("video.mp4"))
            .unwrap()
            .write_all(b"")
            .unwrap();
        File::create(dir.join("notes.txt"))
            .unwrap()
            .write_all(b"")
            .unwrap();

        let choices = choice_from_dir(&dir);
        assert!(
            !choices.contains(&String::from("video.mp4")),
            "mp4 should not appear in choices"
        );
        assert!(
            !choices.contains(&String::from("notes.txt")),
            "txt should not appear in choices"
        );
    }

    #[test]
    fn choice_from_dir_excludes_files_without_extension() {
        let dir = make_temp_dir("chronosub_test_no_ext");
        File::create(dir.join("Makefile"))
            .unwrap()
            .write_all(b"")
            .unwrap();

        let choices = choice_from_dir(&dir);
        assert!(
            !choices.contains(&String::from("Makefile")),
            "File without extension should not appear in choices"
        );
    }

    #[test]
    fn choice_from_dir_extension_match_is_case_insensitive() {
        let dir = make_temp_dir("chronosub_test_case");
        File::create(dir.join("CAPS.SRT"))
            .unwrap()
            .write_all(b"")
            .unwrap();

        let choices = choice_from_dir(&dir);
        assert!(
            choices.contains(&String::from("CAPS.SRT")),
            "Expected 'CAPS.SRT' (uppercase extension) in choices, got: {:?}",
            choices
        );
    }
}
