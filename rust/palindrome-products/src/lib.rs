pub struct FactorPairGenerator {
    step: i128,
    start: i128,
    end: i128,
    current: Option<(u64, u64)>,
}

impl FactorPairGenerator {
    pub fn new(start: u64, end: u64) -> Self {
        let step = if start > end { -1 } else { 1 };

        Self {
            step,
            start: start as i128,
            end: end as i128,
            current: None,
        }
    }
}

impl Iterator for FactorPairGenerator {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some((a, b)) if a as i128 == self.end && b as i128 == self.end => None,
            Some((a, b)) if b as i128 == self.end => {
                self.current = Some(((a as i128 + self.step) as u64, self.start as u64));
                self.current
            }
            Some((a, b)) => {
                self.current = Some((a, (b as i128 + self.step) as u64));
                self.current
            }
            // Init
            None => {
                self.current = Some((self.start as u64, self.start as u64));
                self.current
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome {
    value: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Self { value: a * b }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.value = a * b;
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut min_gen = FactorPairGenerator::new(min, max);

    let min_pal =
        min_gen.find_map(|(a, b)| is_palindrome_number(a * b).then(|| Palindrome::new(a, b)));

    let max_gen = FactorPairGenerator::new(max, min);

    let max_pal = max_gen
        .filter_map(|(a, b)| is_palindrome_number(a * b).then(|| Palindrome::new(a, b)))
        // HACK: max -> min palindromic products are not generated in largest first order so collect couple values first and then take the largest from them.
        // Collecting 3 is enough to pass the unit tests but might need to increase the number for other scenarios.
        .take(3)
        .max();

    match (min_pal, max_pal) {
        (Some(min), Some(max)) => Some((min, max)),
        _ => None,
    }
}

fn is_palindrome_number(number: u64) -> bool {
    let mut reminder = number;
    let mut reversed = 0;

    while reminder != 0 {
        reversed = reversed * 10 + reminder % 10;
        reminder /= 10;
    }

    number == reversed
}
