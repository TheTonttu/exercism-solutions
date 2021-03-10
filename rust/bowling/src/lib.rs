use std::iter::once;

const MAX_PINS_PER_ROLL: u16 = 10;
const FRAMES_PER_GAME: usize = 10;
const STRIKE_ROLL: u16 = 10;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
enum FrameType {
    Open,
    Spare,
    Strike
}

#[derive(Debug)]
struct Frame {
    rolls: Vec<u16>,
    frame_type: FrameType
}

impl Frame {
    fn new(rolls: &[u16]) -> Self {
        let frame_type = match rolls {
            [r1] if *r1 == 10 => FrameType::Strike,
            [r1, r2] if *r1 + *r2 == 10 => FrameType::Spare,
            [r1, r2] if *r1 + *r2 < 10 => FrameType::Open,
            _ => unimplemented!("Unsupported {:?}", rolls)
        };
        Self {
            rolls: Vec::from(rolls),
            frame_type
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
            pending_rolls: vec![],
            frames: Vec::with_capacity(FRAMES_PER_GAME as usize),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // TODO: Get rid of collect
        match try_create_frame(self.pending_rolls.iter().chain(once(&pins)).copied().collect::<Vec<u16>>(), self.frames.len() + 1)? {
            Some(frame) => {
                self.frames.push(frame);
                self.pending_rolls.clear();
                Ok(())
            },
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

                self.frames.iter().enumerate().for_each(|(e,f)| println!("{}. {:?}", e, f));

                // WTF...
                for (index, frame) in self.frames.iter().enumerate() {
                    let roll_points: u16 = frame.rolls.iter().sum();
                    let bonus = match frame.frame_type {
                        FrameType::Open => 0,
                        // Spare => bonus from next roll
                        FrameType::Spare => {
                            match self.frames.get(index + 1) {
                                Some(next_frame) => *next_frame.rolls.first().unwrap(),
                                None => 0,
                            }
                        },
                        // Strike => bonus from next 2 rolls
                        FrameType::Strike => {
                            match self.frames.get(index + 1) {
                                Some(next_frame) => match next_frame.frame_type {
                                    FrameType::Open | FrameType::Spare => next_frame.rolls.iter().sum::<u16>(),
                                    // If next frame is strike then it only has 1 roll and we need to take the first roll from consecutive frame
                                    FrameType::Strike => {
                                        match self.frames.get(index + 2) {
                                            Some(frame_after_next) => next_frame.rolls.iter().sum::<u16>() + *frame_after_next.rolls.first().unwrap(),
                                            None => next_frame.rolls.iter().sum::<u16>(),
                                        }
                                    }
                                },
                                None => 0,
                            }
                        },
                    };

                    score += roll_points + bonus;
                }

                Some(score)
            },
            _ => None,
        }
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

fn try_create_frame<T: AsRef<[u16]>>(rolls: T, frame_number: usize) -> Result<Option<Frame>, Error> {
    let rolls = rolls.as_ref();
    println!("{:?}", rolls);
    match (frame_number, rolls) {
        (frame_number, _) if frame_number > FRAMES_PER_GAME => Err(Error::GameComplete),
        (_, [r1, r2]) if *r1 + *r2 > MAX_PINS_PER_ROLL => {
            Err(Error::NotEnoughPinsLeft)
        },
        (_, [r1]) if *r1 > MAX_PINS_PER_ROLL => {
            Err(Error::NotEnoughPinsLeft)
        },
        // Continue collecting
        (_, [r1]) if *r1 < STRIKE_ROLL => {
            Ok(None)
        }
        _ => Ok(Some(Frame::new(rolls)))
    }
}
