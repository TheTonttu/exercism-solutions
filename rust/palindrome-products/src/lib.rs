use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
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
    let palindromes = (min..=max)
        .chain(min..=max)
        // collect unique factor permutations
        .permutations(2)
        .unique()
        // filter palindromic products
        .filter_map(|x| match x.as_slice() {
            [a, b] => is_palindrome_number((*a) * (*b)).then(|| (*a, *b)),
            _ => None,
        })
        // collect palindromes
        .map(|(a, b)| Palindrome::new(a, b))
        .collect::<Vec<_>>();

    let min = palindromes.iter().min();
    let max = palindromes.iter().max();

    match (min, max) {
        (Some(min), Some(max)) => Some((*min, *max)),
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
