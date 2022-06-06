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

macro_rules! planet {
    ($planet_id:ident, $year_length:literal) => {
        impl Planet for $planet_id {
            const YEAR_LENGTH: f64 = $year_length;
        }
    };
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);