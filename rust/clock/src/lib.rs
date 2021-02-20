use core::fmt;

const HOURS_IN_DAY: i32 = 24;
const MINUTES_IN_HOUR: i32 = 60;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (extracted_hours, extracted_minutes) = extract_hours_from_minutes(minutes);

        let normalized_hours = match hours + extracted_hours {
            h if h > 0 => h % HOURS_IN_DAY,
            h if h < 0 => (HOURS_IN_DAY - (h.abs() % HOURS_IN_DAY)) % HOURS_IN_DAY,
            _ => 0,
        };

        Clock {
            hours: normalized_hours,
            minutes: extracted_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

fn extract_hours_from_minutes(minutes: i32) -> (i32, i32) {
    match minutes {
        min if min > 0 => (minutes / MINUTES_IN_HOUR, minutes % MINUTES_IN_HOUR),
        min if min < 0 => {
            let hours = -(min.abs() as f32 / MINUTES_IN_HOUR as f32).ceil() as i32;
            (hours, (MINUTES_IN_HOUR + (min % MINUTES_IN_HOUR)) % MINUTES_IN_HOUR)
        },
        _ => (0, 0),
    }
}
