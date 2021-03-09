const MAX_PINS_PER_ROLL: u16 = 10;
const FRAMES_PER_GAME: u16 = 10;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    roll_count: u16,
    frame_count: u16,
    score: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            roll_count: 0,
            score: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match (self.frame_count, pins) {
            (frame_count, _) if frame_count >= FRAMES_PER_GAME => Err(Error::GameComplete),
            (_, pins) if (0..=MAX_PINS_PER_ROLL).contains(&pins) => {
                self.roll_count += 1;
                // TODO: Proper open frame management
                self.frame_count = self.roll_count / 2;
                self.score += pins;
                Ok(())
            }
            _ => Err(Error::NotEnoughPinsLeft),
        }
    }

    pub fn score(&self) -> Option<u16> {
        match self.frame_count {
            FRAMES_PER_GAME => Some(self.score),
            _ => None,
        }
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
