use crate::Direction::{North, East, South, West};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
pub struct Robot {
    dir: Direction,
    x_pos: i32,
    y_pos: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            dir: d,
            x_pos: x,
            y_pos: y,
        }
    }

    pub fn turn_right(mut self) -> Self {
        let new_dir = match self.dir {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        self.dir = new_dir;
        self
    }

    pub fn turn_left(mut self) -> Self {
        let new_dir = match self.dir {
            North => West,
            East => North,
            South => East,
            West => South,
        };
        self.dir = new_dir;
        self
    }

    pub fn advance(mut self) -> Self {
        let movement = match self.dir {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        };

        let (mov_x, mov_y) = movement;
        self.x_pos += mov_x;
        self.y_pos += mov_y;

        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        // Probably not very efficient if we are cloning all the time...
        for c in instructions.chars() {
            let wut = match c {
                'A' => self.advance(),
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                _ => self,
            };
            self = wut;
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x_pos, self.y_pos)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
