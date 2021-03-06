use num_traits::Num;
use std::iter::Sum;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<'a, T: 'a + Num + PartialOrd + Copy + Sum> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        match is_valid_sides(sides) {
            true => Some(Triangle { sides }),
            false => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let (a, b, c) = (self.sides[0], self.sides[1], self.sides[2]);
        a == b && b == c

        // self.sides.windows(2).all(|window| match window {
        //     [s1, s2] => s1 == s2,
        //     _ => false,
        // })
    }

    pub fn is_scalene(&self) -> bool {
        !(self.is_equilateral() || self.is_isosceles())
    }

    pub fn is_isosceles(&self) -> bool {
        let (a, b, c) = (self.sides[0], self.sides[1], self.sides[2]);
        (a == b) || (b == c) || (a == c)
    }
}

fn is_valid_sides<'a, T: 'a + Num + PartialOrd + Copy + Sum>(sides: [T; 3]) -> bool {
    // a == 0 || b == 0 || c == 0
    if sides.iter().any(|s| (*s).is_zero()) {
        return false;
    }

    let perimeter = sides.iter().copied().sum::<T>();
    // a+b > c == (a+b+c)-c > c
    sides.iter().all(|s| perimeter - *s >= *s)
}
