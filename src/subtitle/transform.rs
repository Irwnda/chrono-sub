use crate::subtitle::time::{Direction, SubTime};
use regex::Regex;
use std::path::Path;
use std::sync::LazyLock;

pub(crate) fn transform_subtitle(
    content: &str,
    sub_time: &SubTime,
    direction: &Direction,
    separator: char,
) -> Result<Vec<String>, String> {
    let mut new_content: Vec<String> = vec![];

    for line in content.lines() {
        match extract_timestamp_line(line) {
            Ok((start, end)) => {
                let start_time = SubTime::from_str(start).unwrap();
                let end_time = SubTime::from_str(end).unwrap();

                let new_start_time = start_time.calculate(sub_time, direction)?;
                let new_end_time = end_time.calculate(sub_time, direction)?;

                let mut new_line = String::new();
                new_line.push_str(&format!("{} -->", new_start_time.to_string(separator)));
                new_line.push_str(&format!(" {}", new_end_time.to_string(separator)));

                new_content.push(new_line);
            }
            Err(_) => {
                let mut new_line = String::new();
                new_line.push_str(line);

                new_content.push(new_line);
            }
        }
    }

    Ok(new_content)
}

fn extract_timestamp_line(line: &str) -> Result<(&str, &str), bool> {
    if let Some((start, end)) = line.split_once("-->")
        && TIME_REGEX.is_match(start.trim())
        && TIME_REGEX.is_match(end.trim())
    {
        return Ok((start.trim(), end.trim()));
    }

    Err(false)
}

pub(crate) fn separator(file: &Path) -> Option<char> {
    match file.extension() {
        Some(ext) => {
            let extension = ext.to_str().unwrap().to_lowercase();
            if extension == "srt" {
                Some(',')
            } else if extension == "vtt" {
                Some('.')
            } else {
                None
            }
        }
        None => None,
    }
}

static TIME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\d{2}:\d{2}:\d{2}[.,]\d{2,3}$").unwrap());

pub(crate) fn validate_time(time: &str) -> bool {
    TIME_REGEX.is_match(time)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // ── transform_subtitle ───────────────────────────────────────────────────

    #[test]
    fn test_transform_subtitle_valid() {
        let content = "1\n00:00:01.000 --> 00:00:02.000\nHello";
        let sub_time = SubTime::from_str("00:00:01.000").unwrap();
        let result = transform_subtitle(content, &sub_time, &Direction::Forward, '.').unwrap();
        assert!(result.len() > 0);
    }

    // ── extract_timestamp_line ───────────────────────────────────────────────

    #[test]
    fn test_extract_timestamp_line_valid() {
        let line = "00:00:01.000 --> 00:00:02.000";
        assert_eq!(
            extract_timestamp_line(line),
            Ok(("00:00:01.000", "00:00:02.000"))
        );
    }

    #[test]
    fn test_extract_timestamp_line_invalid() {
        assert!(extract_timestamp_line("not a timestamp").is_err());
        assert!(extract_timestamp_line("00:00:01 --> invalid").is_err());
    }

    // ── separator ────────────────────────────────────────────────────────────

    #[test]
    fn test_separator_detection() {
        assert_eq!(separator(&PathBuf::from("test.srt")), Some(','));
        assert_eq!(separator(&PathBuf::from("test.vtt")), Some('.'));
        assert_eq!(separator(&PathBuf::from("test.txt")), None);
    }

    // ── validate_time ────────────────────────────────────────────────────────

    #[test]
    fn valid_time_format_accepted() {
        assert!(validate_time("00:00:01.500"));
        assert!(validate_time("01:30:00.00"));
    }

    #[test]
    fn invalid_time_format_rejected() {
        assert!(!validate_time("1:2:3"));
        assert!(!validate_time("00:00:01"));
        assert!(!validate_time("abc"));
        assert!(!validate_time(""));
    }
}
