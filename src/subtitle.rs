use crossterm::style::{Color, Stylize, style};
use inquire::{Select, Text, validator::Validation};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[derive(Debug)]
struct SubTime {
    hours: u64,
    minutes: u64,
    seconds: u64,
    milliseconds: u64,
}

enum Encoding {
    Utf8,
    Utf16Le,
    Windows1252,
}

enum Direction {
    Forward,
    Backward,
}

impl SubTime {
    fn from_str(time_str: &str) -> Option<Self> {
        let clean = time_str.trim();
        let parts: Vec<&str> = clean.splitn(2, |c| c == '.' || c == ',').collect();

        if parts.len() != 2 {
            return None;
        };

        let hms_parts: Vec<&str> = parts[0].split(':').collect();
        if hms_parts.len() != 3 {
            return None;
        };

        let hours = hms_parts[0].parse::<u64>().ok()?;
        let minutes = hms_parts[1].parse::<u64>().ok()?;
        let seconds = hms_parts[2].parse::<u64>().ok()?;

        let milliseconds = {
            let mut ms_str = parts[1].to_string();
            ms_str.push_str("000");
            ms_str.truncate(3);
            ms_str.parse::<u64>().ok()?
        };

        Some(Self {
            hours,
            minutes,
            seconds,
            milliseconds,
        })
    }

    fn to_millisecond(&self) -> u64 {
        let total_s = (self.hours * 60 * 60) + (self.minutes * 60) + self.seconds;

        total_s * 1000 + self.milliseconds
    }

    fn from_millisecond(ms: u64) -> Self {
        let total_s = ms / 1000;
        let hours = total_s / 3600;
        let minutes = (total_s % 3600) / 60;
        let seconds = total_s % 60;
        let milliseconds = ms % 1000;

        Self {
            hours,
            minutes,
            seconds,
            milliseconds,
        }
    }

    fn add(&self, other: &SubTime) -> Self {
        Self::from_millisecond(self.to_millisecond() + other.to_millisecond())
    }

    fn sub(&self, other: &SubTime) -> Result<Self, String> {
        if self.to_millisecond() < other.to_millisecond() {
            return Err(String::from("Subtitle timestamp cannot be negative"));
        }

        Ok(Self::from_millisecond(
            self.to_millisecond() - other.to_millisecond(),
        ))
    }

    fn calculate(&self, other: &Self, direction: &Direction) -> Result<Self, String> {
        match direction {
            Direction::Forward => Ok(self.add(other)),
            Direction::Backward => self.sub(other),
        }
    }

    fn to_string(&self, mill_separator: char) -> String {
        format!(
            "{:02}:{:02}:{:02}{mill_separator}{:03}",
            self.hours, self.minutes, self.seconds, self.milliseconds
        )
    }
}

pub fn process(file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let direction = prompt_direction();
    let time_adjustment = adjustment_duration();

    let sub_time = match SubTime::from_str(&time_adjustment) {
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

    let separator = match separator(&file) {
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
    let (sub_content, encoding) = read_subtitle_file(&file)?;
    let new_content = match transform_subtitle(&sub_content, &sub_time, &direction, separator) {
        Ok(result) => result,
        Err(e) => {
            println!("{}", style(&e).with(Color::Red).bold());
            return Err(e.into());
        }
    };

    Ok(save_file(&file, &new_content, encoding))
}

fn read_subtitle_file(file: &PathBuf) -> Result<(String, Encoding), Box<dyn std::error::Error>> {
    let bytes = fs::read(file)?;

    if bytes.starts_with(&[0xFF, 0xFE]) {
        let utf16: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        return Ok((String::from_utf16(&utf16)?, Encoding::Utf16Le));
    }

    if let Ok(s) = std::str::from_utf8(&bytes) {
        return Ok((s.to_string(), Encoding::Utf8));
    }

    let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(&bytes);
    Ok((decoded.into_owned(), Encoding::Windows1252))
}

fn prompt_direction() -> Direction {
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

fn separator(file: &PathBuf) -> Option<char> {
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

fn transform_subtitle(
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

                let new_start_time = match start_time.calculate(&sub_time, &direction) {
                    Ok(st) => st,
                    Err(e) => return Err(e),
                };
                let new_end_time = match end_time.calculate(&sub_time, &direction) {
                    Ok(st) => st,
                    Err(e) => return Err(e),
                };

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

fn adjustment_duration() -> String {
    Text::new("Enter the adjustment duration:")
        .with_placeholder("hh:mm:ss,ms (e.g., 00:00:01,500 for 1.5 seconds)")
        .with_help_message(
            "Format must be hours:minutes:seconds,milliseconds (or you can use . for milliseconds)",
        )
        .with_validator(move |input: &str| {
            if validate_time(input) {
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

static TIME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\d{2}:\d{2}:\d{2}[.,]\d{2,3}$").unwrap());

fn validate_time(time: &str) -> bool {
    TIME_REGEX.is_match(time)
}

fn extract_timestamp_line(line: &str) -> Result<(&str, &str), bool> {
    if let Some((start, end)) = line.split_once("-->") {
        if TIME_REGEX.is_match(start.trim()) && TIME_REGEX.is_match(end.trim()) {
            return Ok((start.trim(), end.trim()));
        }
    }

    Err(false)
}

fn prompt_naming(file: &PathBuf) -> PathBuf {
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

fn save_file(file: &PathBuf, content: &Vec<String>, encoding: Encoding) {
    let location = prompt_naming(file);
    let text = content.join("\n");

    let bytes: Vec<u8> = match encoding {
        Encoding::Utf8 => text.into_bytes(),
        Encoding::Utf16Le => {
            let mut out = vec![0xFF, 0xFE]; // BOM
            for unit in text.encode_utf16() {
                out.extend_from_slice(&unit.to_le_bytes());
            }
            out
        }
        Encoding::Windows1252 => {
            let (encoded, _, _) = encoding_rs::WINDOWS_1252.encode(&text);
            encoded.into_owned()
        }
    };

    fs::write(location, bytes).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Option<u64> {
        SubTime::from_str(s).map(|st| st.to_millisecond())
    }

    // ── SubTime::from_str + to_millisecond ───────────────────────────────────

    #[test]
    fn parse_time_one_hour() {
        assert_eq!(parse("01:00:00.000"), Some(3_600_000));
    }

    #[test]
    fn parse_time_one_minute() {
        assert_eq!(parse("00:01:00.000"), Some(60_000));
    }

    #[test]
    fn parse_time_one_second() {
        assert_eq!(parse("00:00:01.000"), Some(1_000));
    }

    #[test]
    fn parse_time_milliseconds_only() {
        assert_eq!(parse("00:00:00.500"), Some(500));
    }

    #[test]
    fn parse_time_mixed_values() {
        let expected = (1 * 3600 + 23 * 60 + 45) * 1_000 + 678;
        assert_eq!(parse("01:23:45.678"), Some(expected));
    }

    #[test]
    fn parse_time_zero() {
        assert_eq!(parse("00:00:00.000"), Some(0));
    }

    #[test]
    fn parse_time_trims_whitespace() {
        assert_eq!(parse("  00:00:01.000  "), Some(1_000));
    }

    #[test]
    fn parse_time_two_digit_ms_pads_correctly() {
        assert_eq!(parse("00:00:00.50"), Some(500));
    }

    #[test]
    fn parse_time_one_digit_ms_pads_correctly() {
        assert_eq!(parse("00:00:00.5"), Some(500));
    }

    #[test]
    fn parse_time_empty_string_returns_none() {
        assert_eq!(parse(""), None);
    }

    #[test]
    fn parse_time_no_dot_returns_none() {
        assert_eq!(parse("00:00:01"), None);
    }

    #[test]
    fn parse_time_wrong_hms_parts_returns_none() {
        assert_eq!(parse("00:01.000"), None);
    }

    #[test]
    fn parse_time_non_numeric_returns_none() {
        assert_eq!(parse("ab:00:00.000"), None);
        assert_eq!(parse("00:cd:00.000"), None);
        assert_eq!(parse("00:00:ef.000"), None);
        assert_eq!(parse("00:00:00.xyz"), None);
    }

    #[test]
    fn parse_time_multiple_dots_returns_none() {
        assert_eq!(parse("00:00:01.5.0"), None);
    }

    // ── SubTime fields ───────────────────────────────────────────────────────

    #[test]
    fn from_str_populates_fields_correctly() {
        let st = SubTime::from_str("01:23:45.678").unwrap();
        assert_eq!(st.hours, 1);
        assert_eq!(st.minutes, 23);
        assert_eq!(st.seconds, 45);
        assert_eq!(st.milliseconds, 678);
    }

    // ── SubTime::from_millisecond ─────────────────────────────────────────────

    #[test]
    fn from_millisecond_zero() {
        let st = SubTime::from_millisecond(0);
        assert_eq!(st.hours, 0);
        assert_eq!(st.minutes, 0);
        assert_eq!(st.seconds, 0);
        assert_eq!(st.milliseconds, 0);
    }

    #[test]
    fn from_millisecond_populates_fields_correctly() {
        let ms = (1 * 3600 + 23 * 60 + 45) * 1_000 + 678;
        let st = SubTime::from_millisecond(ms);
        assert_eq!(st.hours, 1);
        assert_eq!(st.minutes, 23);
        assert_eq!(st.seconds, 45);
        assert_eq!(st.milliseconds, 678);
    }

    #[test]
    fn from_millisecond_milliseconds_only() {
        let st = SubTime::from_millisecond(999);
        assert_eq!(st.hours, 0);
        assert_eq!(st.minutes, 0);
        assert_eq!(st.seconds, 0);
        assert_eq!(st.milliseconds, 999);
    }

    #[test]
    fn from_millisecond_round_trips_with_to_millisecond() {
        let original_ms = 4_567_890_u64;
        let st = SubTime::from_millisecond(original_ms);
        assert_eq!(st.to_millisecond(), original_ms);
    }

    // ── SubTime::add ──────────────────────────────────────────────────────────

    #[test]
    fn add_basic() {
        let a = SubTime::from_str("00:00:01.000").unwrap();
        let b = SubTime::from_str("00:00:02.000").unwrap();
        let result = a.add(&b);
        assert_eq!(result.to_millisecond(), 3_000);
    }

    #[test]
    fn add_milliseconds_carry_into_seconds() {
        let a = SubTime::from_str("00:00:00.600").unwrap();
        let b = SubTime::from_str("00:00:00.700").unwrap();
        let result = a.add(&b);
        assert_eq!(result.seconds, 1);
        assert_eq!(result.milliseconds, 300);
    }

    #[test]
    fn add_with_zero() {
        let a = SubTime::from_str("01:23:45.678").unwrap();
        let b = SubTime::from_millisecond(0);
        let result = a.add(&b);
        assert_eq!(result.to_millisecond(), a.to_millisecond());
    }

    #[test]
    fn add_hours_minutes_seconds() {
        let a = SubTime::from_str("01:30:00.000").unwrap();
        let b = SubTime::from_str("00:45:00.000").unwrap();
        let result = a.add(&b);
        assert_eq!(result.hours, 2);
        assert_eq!(result.minutes, 15);
        assert_eq!(result.seconds, 0);
        assert_eq!(result.milliseconds, 0);
    }

    // ── SubTime::sub ──────────────────────────────────────────────────────────

    #[test]
    fn sub_basic() {
        let a = SubTime::from_str("00:00:05.000").unwrap();
        let b = SubTime::from_str("00:00:02.000").unwrap();
        let result = a.sub(&b).unwrap();
        assert_eq!(result.to_millisecond(), 3_000);
    }

    #[test]
    fn sub_result_is_zero() {
        let a = SubTime::from_str("00:01:00.000").unwrap();
        let b = SubTime::from_str("00:01:00.000").unwrap();
        let result = a.sub(&b).unwrap();
        assert_eq!(result.to_millisecond(), 0);
    }

    #[test]
    fn sub_milliseconds_borrow_from_seconds() {
        let a = SubTime::from_str("00:00:01.000").unwrap();
        let b = SubTime::from_str("00:00:00.300").unwrap();
        let result = a.sub(&b).unwrap();
        assert_eq!(result.seconds, 0);
        assert_eq!(result.milliseconds, 700);
    }

    #[test]
    fn sub_underflow_returns_err() {
        let a = SubTime::from_str("00:00:01.000").unwrap();
        let b = SubTime::from_str("00:00:02.000").unwrap();
        let result = a.sub(&b);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Subtitle timestamp cannot be negative");
    }

    // ── SubTime::to_string ───────────────────────────────────────────────────

    #[test]
    fn to_string_formats_correctly() {
        let st = SubTime {
            hours: 1,
            minutes: 2,
            seconds: 3,
            milliseconds: 456,
        };
        assert_eq!(st.to_string('.'), "01:02:03.456");
        assert_eq!(st.to_string(','), "01:02:03,456");
    }

    // ── SubTime::calculate ───────────────────────────────────────────────────

    #[test]
    fn calculate_forward() {
        let start = SubTime::from_str("00:00:01.000").unwrap();
        let adjustment = SubTime::from_str("00:00:01.000").unwrap();
        let result = start.calculate(&adjustment, &Direction::Forward).unwrap();
        assert_eq!(result.to_millisecond(), 2000);
    }

    #[test]
    fn calculate_backward() {
        let start = SubTime::from_str("00:00:02.000").unwrap();
        let adjustment = SubTime::from_str("00:00:01.000").unwrap();
        let result = start.calculate(&adjustment, &Direction::Backward).unwrap();
        assert_eq!(result.to_millisecond(), 1000);
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

    // ── transform_subtitle ───────────────────────────────────────────────────

    #[test]
    fn test_transform_subtitle_valid() {
        let content = "1\n00:00:01.000 --> 00:00:02.000\nHello";
        let sub_time = SubTime::from_str("00:00:01.000").unwrap();
        let result = transform_subtitle(content, &sub_time, &Direction::Forward, '.').unwrap();
        assert!(result.len() > 0);
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
