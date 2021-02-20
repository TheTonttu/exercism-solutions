use core::fmt;

const HOURS_IN_DAY: i32 = 24;
const MINUTES_IN_HOUR: i32 = 60;
const MINUTES_IN_DAY: i32 = HOURS_IN_DAY * MINUTES_IN_HOUR;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let collective_minutes = hours * MINUTES_IN_HOUR + minutes;
        let normalized_minutes = ((collective_minutes % MINUTES_IN_DAY)
            // Make sure value is positive
            + MINUTES_IN_DAY)
            // Keep value within 24 h
            % MINUTES_IN_DAY;

        Clock {
            hours: normalized_minutes / MINUTES_IN_HOUR,
            minutes: normalized_minutes % MINUTES_IN_HOUR,
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
