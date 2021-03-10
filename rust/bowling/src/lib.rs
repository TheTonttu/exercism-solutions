use std::iter::once;

const MAX_PINS_PER_ROLL: u16 = 10;
const FRAMES_PER_GAME: usize = 10;
const STRIKE_THROW: u16 = 10;
const SPARE_THROW: u16 = 10;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
enum FrameType {
    Open,
    Spare,
    Strike,
    LastFrame,
}

#[derive(Debug)]
struct Frame {
    rolls: Vec<u16>,
    frame_type: FrameType,
}

impl Frame {
    fn new(rolls: &[u16]) -> Self {
        let frame_type = match rolls {
            [r1] if *r1 == STRIKE_THROW => FrameType::Strike,
            [r1, r2] if *r1 + *r2 == SPARE_THROW => FrameType::Spare,
            [r1, r2] if *r1 + *r2 < SPARE_THROW => FrameType::Open,
            _ => FrameType::LastFrame,
        };

        Self {
            rolls: Vec::from(rolls),
            frame_type,
        }
    }
}

pub struct BowlingGame {
    pending_rolls: Vec<u16>,
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            pending_rolls: Vec::new(),
            frames: Vec::with_capacity(FRAMES_PER_GAME),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match try_create_frame(
            self.pending_rolls
                .iter()
                .chain(once(&pins))
                .copied()
                .collect::<Vec<u16>>(),
            self.frames.len() + 1,
        )? {
            Some(frame) => {
                self.frames.push(frame);
                self.pending_rolls.clear();
                Ok(())
            }
            None => {
                self.pending_rolls.push(pins);
                Ok(())
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        match self.frames.len() {
            FRAMES_PER_GAME => {
                let mut score = 0;

                for (index, frame) in self.frames.iter().enumerate() {
                    let bonus_roll_count = match frame.frame_type {
                        FrameType::Open | FrameType::LastFrame => 0,
                        FrameType::Spare => 1,
                        FrameType::Strike => 2,
                    };
                    // Sum rolls from following frames that count toward the frame bonus.
                    let bonus_points = self
                        .frames
                        .iter()
                        .skip(index + 1)
                        .flat_map(|f| f.rolls.iter())
                        .take(bonus_roll_count)
                        .sum::<u16>();

                    let roll_points: u16 = frame.rolls.iter().sum();

                    score += roll_points + bonus_points;
                }

                Some(score)
            }
            _ => None,
        }
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

fn try_create_frame<T: AsRef<[u16]>>(
    rolls: T,
    frame_number: usize,
) -> Result<Option<Frame>, Error> {
    let rolls = rolls.as_ref();

    if rolls.iter().any(|r| *r > MAX_PINS_PER_ROLL) {
        return Err(Error::NotEnoughPinsLeft);
    }

    match (frame_number, rolls) {
        (frame_number, _) if frame_number > FRAMES_PER_GAME => Err(Error::GameComplete),
        // Special last frame
        (frame_number, last_rolls) if frame_number == FRAMES_PER_GAME => {
            match last_rolls {
                // Strike followed by knocking too many pins with fill ball
                [r1, r2, r3]
                    if *r1 == STRIKE_THROW && *r2 < STRIKE_THROW && (*r2 + *r3) > SPARE_THROW =>
                {
                    Err(Error::NotEnoughPinsLeft)
                }
                // Fill ball
                [r1, r2, _] if *r1 == STRIKE_THROW || *r1 + *r2 == SPARE_THROW => {
                    Ok(Some(Frame::new(rolls)))
                }
                // Last frame is open
                [r1, r2] if *r1 + *r2 < SPARE_THROW => Ok(Some(Frame::new(rolls))),
                // Else continue collecting
                _ => Ok(None),
            }
        }
        // Spare has extra pins
        (_, [r1, r2]) if *r1 + *r2 > SPARE_THROW => Err(Error::NotEnoughPinsLeft),
        // Continue collecting
        (_, [r1]) if *r1 < STRIKE_THROW => Ok(None),
        // Else end frame
        _ => Ok(Some(Frame::new(rolls))),
    }
}
