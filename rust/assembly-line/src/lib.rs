const PRODUCTION_RATE_PER_HOUR: f64 = 221.0;
const MINUTES_IN_HOUR: f64 = 60.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (speed as f64) * PRODUCTION_RATE_PER_HOUR *  success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / MINUTES_IN_HOUR) as u32
}

fn success_rate(speed: u8) -> f64 {
    match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0
    }
}
