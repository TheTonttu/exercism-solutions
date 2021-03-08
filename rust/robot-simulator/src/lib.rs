use crate::Direction::{East, North, South, West};

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

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
            West => (-1, 0)
        };

        self.x_pos += movement.0;
        self.y_pos += movement.1;

        self
    }

    pub fn instructions(self, instructions: &str) -> Self {
        unimplemented!(
            "Follow the given sequence of instructions: {}",
            instructions
        )
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x_pos, self.y_pos)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
