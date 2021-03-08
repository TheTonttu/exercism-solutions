// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

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

    pub fn turn_right(self) -> Self {
        unimplemented!()
    }

    pub fn turn_left(self) -> Self {
        unimplemented!()
    }

    pub fn advance(self) -> Self {
        unimplemented!()
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
