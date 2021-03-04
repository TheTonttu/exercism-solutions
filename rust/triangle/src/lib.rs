use std::collections::HashSet;

pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        match sides.iter().any(|s| *s == 0) {
            true => None,
            false => Some(Triangle { sides }),
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().min() == self.sides.iter().max()
    }

    pub fn is_scalene(&self) -> bool {
        self.sides.iter().count() == self.sides.iter().collect::<HashSet<_>>().len()
    }

    pub fn is_isosceles(&self) -> bool {
        unimplemented!("Determine if the Triangle is isosceles.");
    }
}
