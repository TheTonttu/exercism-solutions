#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl Duration {
    const SECONDS_IN_YEAR_ON_EARTH: f64 = 60.0 * 60.0 * 24.0 * 365.25;

    fn earth_years(&self) -> f64 {
        (self.seconds as f64) / Self::SECONDS_IN_YEAR_ON_EARTH
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    /// Planet year length related to earth's ~365.25 day years.
    const YEAR_LENGTH: f64;
    /// Multiplier based on `YEAR_LENGTH`. Should not be overridden.
    const YEAR_MULTIPLIER: f64 = 1.0 / Self::YEAR_LENGTH;

    fn years_during(d: &Duration) -> f64 {
        d.earth_years() * Self::YEAR_MULTIPLIER
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const YEAR_LENGTH: f64 = 0.2408467;
}
impl Planet for Venus {
    const YEAR_LENGTH: f64 = 0.61519726;
}
impl Planet for Earth {
    const YEAR_LENGTH: f64 = 1.0;
}
impl Planet for Mars {
    const YEAR_LENGTH: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const YEAR_LENGTH: f64 = 11.862615;
}
impl Planet for Saturn {
    const YEAR_LENGTH: f64 = 29.447498;
}
impl Planet for Uranus {
    const YEAR_LENGTH: f64 = 84.016846;
}
impl Planet for Neptune {
    const YEAR_LENGTH: f64 = 164.79132;
}
