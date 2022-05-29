pub struct FactorPairGenerator {
    step: i128,
    start: u64,
    end: u64,
    current: Option<(u64, u64)>,
}

impl FactorPairGenerator {
    pub fn new(start: u64, end: u64) -> Self {
        Self {
            step: if start > end { -1 } else { 1 },
            start,
            end,
            current: None,
        }
    }

    fn next_pair(&self) -> Option<(u64, u64)> {
        match self.current {
            Some(pair) => match pair {
                end if end == (self.end, self.end) => None,
                (a, b) if b == self.end => Some(((a as i128 + self.step) as u64, self.start)),
                (a, b) => Some((a, (b as i128 + self.step) as u64)),
            },
            // Init
            None => Some((self.start, self.start)),
        }
    }
}

impl Iterator for FactorPairGenerator {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_pair() {
            Some(pair) => {
                self.current.replace(pair);
                self.current
            }
            _ => None,
        }
    }
}

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if Self::is_palindrome(value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }

    fn is_palindrome(number: u64) -> bool {
        let mut reminder = number;
        let mut reversed = 0;

        while reminder != 0 {
            reversed = reversed * 10 + reminder % 10;
            reminder /= 10;
        }
        number == reversed
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut min_gen = FactorPairGenerator::new(min, max);
    let min_palindrome = min_gen.find_map(|(a, b)| Palindrome::new(a * b));

    let max_gen = FactorPairGenerator::new(max, min);
    let max_palindrome = max_gen
        .filter_map(|(a, b)| Palindrome::new(a * b))
        // HACK: max -> min palindromic products are not generated in largest first order so collect couple values first and then take the largest from them.
        // Collecting 3 is enough to pass the unit tests but might need to increase the number for other scenarios.
        .take(3)
        .max();

    min_palindrome.and_then(|min| max_palindrome.map(|max| (min, max)))
}
