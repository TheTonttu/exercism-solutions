const MAX_PINS_PER_ROLL: u16 = 10;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    roll_count: u16,
    score: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            roll_count: 0,
            score: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match pins {
            0..=MAX_PINS_PER_ROLL => {
                self.roll_count += 1;
                self.score += pins;
                Ok(())
            }
            _ => Err(Error::NotEnoughPinsLeft),
        }
    }

    pub fn score(&self) -> Option<u16> {
        match self.roll_count {
            1..=u16::MAX => Some(self.roll_count),
            _ => None,
        }
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
