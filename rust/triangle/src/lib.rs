use std::collections::HashSet;

pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        match is_valid_sides(sides) {
            true => Some(Triangle { sides }),
            false => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().min() == self.sides.iter().max()
    }

    pub fn is_scalene(&self) -> bool {
        self.sides.iter().count() == self.sides.iter().collect::<HashSet<_>>().len()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides.iter().collect::<HashSet<_>>().len() == 2
    }
}

fn is_valid_sides(sides: [u64; 3]) -> bool {
    !sides.iter().any(|s| *s == 0)
}
