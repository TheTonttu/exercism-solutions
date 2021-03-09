const MAX_PINS_PER_ROLL: u16 = 10;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match pins {
            0..=MAX_PINS_PER_ROLL => {
                self.score += pins;
                Ok(())
            }
            _ => Err(Error::NotEnoughPinsLeft)
        }

    }

    pub fn score(&self) -> Option<u16> {
        unimplemented!("Return the score if the game is complete, or None if not.");
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
