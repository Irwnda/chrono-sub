use std::path::PathBuf;
use std::sync::LazyLock;
use inquire::{Select, Text, validator::Validation};
use regex::Regex;

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

    let offset_ms = match parse_time_to_ms(&time_adjustment) {
        Some(ms) => if direction == 0 { ms } else { -ms },
        None => {
            println!("❌ Invalid time format entered. Exiting.");
            return;
        }
    };
}

fn parse_time_to_ms(time: &str) -> Option<i64> {
    let clean = time.trim();
    let parts: Vec<&str> = clean.split('.').collect();

    if parts.len() != 2 { return None };

    let hms_parts: Vec<&str> = parts[0].split(':').collect();
    if hms_parts.len() != 3 { return None };

    let hours = hms_parts[0].parse::<i64>().ok()?;
    let minutes = hms_parts[1].parse::<i64>().ok()?;
    let seconds = hms_parts[2].parse::<i64>().ok()?;

    let milli_seconds = {
        let mut ms_str = parts[1].to_string();
        ms_str.push_str("000");
        ms_str.truncate(3);
        ms_str.parse::<i64>().ok()?
    };

    let total_s = (hours * 60*60) + (minutes * 60) + seconds;

    Some(total_s * 1000 + milli_seconds)
}

fn adjustment_duration() -> String {
    Text::new("Enter the adjustment duration:")
        .with_placeholder("hh:mm:ss.ms (e.g., 00:00:01.500 for 1.5 seconds)")
        .with_help_message("Format must be hours:minutes:seconds.milliseconds")
        .with_validator(move |input: &str| {
            if validate_time(input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Invalid format! Please use hh:mm:ss.ms".into()))
            }
        })
        .prompt()
        .unwrap()
}

static TIME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\d{2}:\d{2}:\d{2}\.\d{2,3}$").unwrap()
});

fn validate_time(time: &str) -> bool {
    TIME_REGEX.is_match(time)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── parse_time_to_ms ─────────────────────────────────────────────────────

    #[test]
    fn parse_time_one_hour() {
        assert_eq!(parse_time_to_ms("01:00:00.000"), Some(3_600_000));
    }

    #[test]
    fn parse_time_one_minute() {
        assert_eq!(parse_time_to_ms("00:01:00.000"), Some(60_000));
    }

    #[test]
    fn parse_time_one_second() {
        assert_eq!(parse_time_to_ms("00:00:01.000"), Some(1_000));
    }

    #[test]
    fn parse_time_milliseconds_only() {
        assert_eq!(parse_time_to_ms("00:00:00.500"), Some(500));
    }

    #[test]
    fn parse_time_mixed_values() {
        let expected = (1 * 3600 + 23 * 60 + 45) * 1000 + 678;
        assert_eq!(parse_time_to_ms("01:23:45.678"), Some(expected));
    }

    #[test]
    fn parse_time_zero() {
        assert_eq!(parse_time_to_ms("00:00:00.000"), Some(0));
    }

    #[test]
    fn parse_time_trims_whitespace() {
        assert_eq!(parse_time_to_ms("  00:00:01.000  "), Some(1_000));
    }

    #[test]
    fn parse_time_two_digit_ms_pads_correctly() {
        assert_eq!(parse_time_to_ms("00:00:00.50"), Some(500));
    }

    #[test]
    fn parse_time_one_digit_ms_pads_correctly() {
        assert_eq!(parse_time_to_ms("00:00:00.5"), Some(500));
    }

    #[test]
    fn parse_time_empty_string_returns_none() {
        assert_eq!(parse_time_to_ms(""), None);
    }

    #[test]
    fn parse_time_no_dot_returns_none() {
        assert_eq!(parse_time_to_ms("00:00:01"), None);
    }

    #[test]
    fn parse_time_wrong_hms_parts_returns_none() {
        assert_eq!(parse_time_to_ms("00:01.000"), None);
    }

    #[test]
    fn parse_time_non_numeric_returns_none() {
        assert_eq!(parse_time_to_ms("ab:00:00.000"), None);
        assert_eq!(parse_time_to_ms("00:cd:00.000"), None);
        assert_eq!(parse_time_to_ms("00:00:ef.000"), None);
        assert_eq!(parse_time_to_ms("00:00:00.xyz"), None);
    }

    #[test]
    fn parse_time_multiple_dots_returns_none() {
        assert_eq!(parse_time_to_ms("00:00:01.5.0"), None);
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

