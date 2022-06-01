use std::fmt;

fn roll_over_for_hours(hours: i32) -> i32 {
    hours % 24
}
fn roll_over_for_minutes(minutes: i32) -> i32 {
    minutes % 60
}

fn format_display_time(value: i32) -> String {
    match value {
        0..=9 => format!("0{}", value),
        _ => value.to_string()
    }
}

pub struct Clock { hours: i32, minutes: i32 }
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // unimplemented!(
        //     "Construct a new Clock from {} hours and {} minutes",
        //     hours,
        //     minutes
        // );
        Self {
            hours: roll_over_for_hours(hours),
            minutes: roll_over_for_minutes(minutes)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // unimplemented!("Add {} minutes to existing Clock time", minutes);
        Self {
            hours: roll_over_for_hours(self.hours),
            minutes: roll_over_for_minutes(self.minutes + minutes)
        }
    }
    
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", format_display_time(self.hours), format_display_time(self.minutes))
    }
}