use chrono::{DateTime, Duration, Utc};
use std::ops::Add;

const GIGASECOND: i64 = 1_000_000_000;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start.add(Duration::seconds(GIGASECOND))
}
