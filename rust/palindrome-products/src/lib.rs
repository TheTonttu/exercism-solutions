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
    let factors: Vec<u64> = (min..=max).chain(min..=max).collect();
    let palindrome_product_factor_pairs = factors
        .iter()
        .permutations(2)
        .unique()
        .filter_map(|x| match x.as_slice() {
            [a, b] => is_palindrome_number((**a) * (**b)).then(|| (**a, **b)),
            _ => None,
        })
        .collect::<Vec<(u64, u64)>>();
    let palindromes = palindrome_product_factor_pairs
        .iter()
        .copied()
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
    // TODO: Remove string conversion
    let str = number.to_string();
    let rev_str = str.chars().rev().collect::<String>();

    str == rev_str
}
