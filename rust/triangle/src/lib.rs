use num_traits::Num;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: Num + PartialOrd + Copy> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        match is_each_side_valid(sides) {
            true => {
                let [a, b, c] = sides;
                Some(Triangle { a, b, c })
            }
            false => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let (a, b, c) = (self.a, self.b, self.c);
        a == b && b == c

        // Old with sides arrays:
        // self.sides.windows(2).all(|window| match window {
        //     [s1, s2] => s1 == s2,
        //     _ => false,
        // })
    }

    pub fn is_scalene(&self) -> bool {
        !(self.is_equilateral() || self.is_isosceles())
    }

    pub fn is_isosceles(&self) -> bool {
        let (a, b, c) = (self.a, self.b, self.c);
        (a == b) || (b == c) || (a == c)
    }
}

fn is_each_side_valid<T: Num + PartialOrd + Copy>(sides: [T; 3]) -> bool {
    // a == 0 || b == 0 || c == 0
    if sides.iter().any(|s| (*s).is_zero()) {
        return false;
    }

    let [a, b, c] = sides;

    // No need for iter Sum trait
    let perimeter = a + b + c;
    // let perimeter = sides.iter().copied().sum::<T>();

    // a+b > c == (a+b+c)-c > c
    sides.iter().all(|s| perimeter - *s >= *s)
}
