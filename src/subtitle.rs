use std::path::PathBuf;
use std::sync::LazyLock;
use inquire::{Select, Text, validator::Validation};
use regex::Regex;

#[derive(Debug)]
struct SubTime {
    hours: u64,
    minutes: u64,
    seconds: u64,
    milliseconds: u64,
}

impl SubTime {
    fn from_str(time_str: &str) -> Option<Self> {
        let clean = time_str.trim();
        let parts: Vec<&str> = clean.split('.').collect();

        if parts.len() != 2 { return None };

        let hms_parts: Vec<&str> = parts[0].split(':').collect();
        if hms_parts.len() != 3 { return None };

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

        Self { hours, minutes, seconds, milliseconds }
    }

    fn add(&self, other: SubTime) -> Self {
        Self::from_millisecond(self.to_millisecond() + other.to_millisecond())
    }

    fn sub(&self, other: SubTime) -> Result<Self, String> {
        if self.to_millisecond() < other.to_millisecond() {
            return Err(String::from("Subtitle timestamp cannot be negative"));
        }

        Ok(Self::from_millisecond(self.to_millisecond() - other.to_millisecond()))
    }
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

    let sub_time = match SubTime::from_str(&time_adjustment) {
        Some(st) => st,
        None => {
            println!("❌ Invalid time format entered. Exiting.");
            return;
        }
    };
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
        let result = a.add(b);
        assert_eq!(result.to_millisecond(), 3_000);
    }

    #[test]
    fn add_milliseconds_carry_into_seconds() {
        let a = SubTime::from_str("00:00:00.600").unwrap();
        let b = SubTime::from_str("00:00:00.700").unwrap();
        let result = a.add(b);
        assert_eq!(result.seconds, 1);
        assert_eq!(result.milliseconds, 300);
    }

    #[test]
    fn add_with_zero() {
        let a = SubTime::from_str("01:23:45.678").unwrap();
        let b = SubTime::from_millisecond(0);
        let result = a.add(b);
        assert_eq!(result.to_millisecond(), a.to_millisecond());
    }

    #[test]
    fn add_hours_minutes_seconds() {
        let a = SubTime::from_str("01:30:00.000").unwrap();
        let b = SubTime::from_str("00:45:00.000").unwrap();
        let result = a.add(b);
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
        let result = a.sub(b).unwrap();
        assert_eq!(result.to_millisecond(), 3_000);
    }

    #[test]
    fn sub_result_is_zero() {
        let a = SubTime::from_str("00:01:00.000").unwrap();
        let b = SubTime::from_str("00:01:00.000").unwrap();
        let result = a.sub(b).unwrap();
        assert_eq!(result.to_millisecond(), 0);
    }

    #[test]
    fn sub_milliseconds_borrow_from_seconds() {
        let a = SubTime::from_str("00:00:01.000").unwrap();
        let b = SubTime::from_str("00:00:00.300").unwrap();
        let result = a.sub(b).unwrap();
        assert_eq!(result.seconds, 0);
        assert_eq!(result.milliseconds, 700);
    }

    #[test]
    fn sub_underflow_returns_err() {
        let a = SubTime::from_str("00:00:01.000").unwrap();
        let b = SubTime::from_str("00:00:02.000").unwrap();
        let result = a.sub(b);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Subtitle timestamp cannot be negative");
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

