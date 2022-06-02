
use std::fmt;

const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;

/**
 * Get the floor of hour from minutes
 */
fn hours_from_minutes(minutes: i32) -> i32 {
    minutes/MINUTES_PER_HOUR
}

/**
 * Roll over for hours
 */
fn roll_over_for_hours(hours: i32) -> i32 {
    if hours % 24 == 0 {
        return 0
    }
    match hours {
        0.. => hours % HOURS_PER_DAY,
        _ => hours % HOURS_PER_DAY + HOURS_PER_DAY
    }
}

/**
 * Roll over for minutes
 */
fn roll_over_for_minutes(minutes: i32) -> i32 {
    if minutes % MINUTES_PER_HOUR == 0 {
        return 0
    }
    match minutes {
        0.. => minutes % MINUTES_PER_HOUR,
        _ => minutes % MINUTES_PER_HOUR + MINUTES_PER_HOUR
    }
}

/**
 * if minutes is a positive integer, then hours will be "hours" plus number of hour converted from "minutes"
 * if minutes is a negative integer, then hours will be the same as above, but it will be minus to "1" in additional
 * Why I need to minus to "1"?
 * For instance, 01:(-40) => 12:20, 02:(-50) => 01:10, 03:(-120) => 01:00
 * That's it!
 */
fn calculate_hours_from_minutes(hours: i32, minutes: i32) -> i32 {
    if minutes < 0 && minutes % MINUTES_PER_HOUR == 0 {
        return roll_over_for_hours(hours + hours_from_minutes(minutes))
    }
    match minutes {
        0.. => roll_over_for_hours(hours + hours_from_minutes(minutes)),
        _ => roll_over_for_hours(hours + hours_from_minutes(minutes) - 1)
    }
}

/**
 * Format of the output should be "hh:mm"
 */
fn format_display_time(value: i32) -> String {
    match value {
        0..=9 => format!("0{}", value).to_string(),
        _ => value.to_string()
    }
}
#[derive(PartialEq, PartialOrd)]
pub struct Clock { hours: i32, minutes: i32 }
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        #[allow(unused_doc_comments)]
        
        let _hours = calculate_hours_from_minutes(hours, minutes);
        Self {
            hours: _hours,
            minutes: roll_over_for_minutes(minutes)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let _hour = calculate_hours_from_minutes(self.hours, self.minutes + minutes);
        Self {
            hours: _hour,
            minutes: roll_over_for_minutes(self.minutes + minutes)
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", format_display_time(self.hours), format_display_time(self.minutes))
    }
}

impl std::fmt::Debug for Clock {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "S2 {{ hours: {:?}, minutes: {:?} }}", self.hours, self.minutes)?;
        Ok(())
    }
}