use num_traits::Num;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Sum;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<'a, T: 'a + Hash + Eq + Num + PartialOrd + Copy + Ord + Sum> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
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

fn is_valid_sides<'a, T: 'a + Num + PartialOrd + Copy + Sum>(sides: [T; 3]) -> bool {
    if sides.iter().any(|s| (*s).is_zero()) {
        return false;
    }

    let perimeter = sides.iter().copied().sum::<T>();
    // a+b > c == (a+b+c)-c > c
    sides.iter().all(|s| perimeter - *s >= *s)
}
