const MAX_PINS_PER_ROLL: u16 = 10;
const FRAMES_PER_GAME: usize = 10;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

struct Frame {
    rolls: Vec<u16>,
}

impl Frame {
    fn new(rolls: &[u16]) -> Self {
        Self {
            rolls: Vec::from(rolls),
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
        match (self.frames.len(), pins) {
            (frame_count, _) if frame_count >= FRAMES_PER_GAME => Err(Error::GameComplete),
            (_, pins) if (0..=MAX_PINS_PER_ROLL).contains(&pins) => {
                self.pending_rolls.push(pins);

                match self.pending_rolls.as_slice() {
                    // Strike
                    [r1, r2, r3] if *r1 == 10 => {
                        self.frames.push(Frame::new(&[*r1, *r2, *r3]));
                        self.pending_rolls.remove(0);
                        println!("Strike!");
                    }
                    // Spare
                    [r1, r2, r3] if *r1 + *r2 == 10 => {
                        self.frames.push(Frame::new(&[*r1, *r2, *r3]));
                        self.pending_rolls.remove(0);
                        self.pending_rolls.remove(0);
                        println!("Spare!");
                    }
                    // Open + 1 roll, e.g. after strike
                    [r1, r2, _] if *r1 + *r2 < 10 => {
                        self.frames.push(Frame::new(&[*r1, *r2]));
                        self.pending_rolls.remove(0);
                        self.pending_rolls.remove(0);
                        println!("Open! + 1 roll");
                    },
                    // Open
                    [r1, r2] if *r1 + *r2 < 10 => {
                        self.frames.push(Frame::new(&[*r1, *r2]));
                        self.pending_rolls.remove(0);
                        self.pending_rolls.remove(0);
                        println!("Open!");
                    }
                    // Rolls don't count as complete frame so let's wait
                    _ => {}
                }

                Ok(())
            }
            _ => Err(Error::NotEnoughPinsLeft),
        }
    }

    pub fn score(&self) -> Option<u16> {
        match self.frames.len() {
            FRAMES_PER_GAME => Some(
                self.frames
                    .iter()
                    .map(|f| f.rolls.iter().sum::<u16>())
                    .sum(),
            ),
            _ => None,
        }
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
