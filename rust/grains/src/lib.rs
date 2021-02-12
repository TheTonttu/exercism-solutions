const MIN_SQUARES: u32 = 1;
const MAX_SQUARES: u32 = 64;

pub fn square(s: u32) -> u64 {
    assert!(MIN_SQUARES <= s && s <= MAX_SQUARES, format!("Square must be between {min} and {max}", min = MIN_SQUARES, max = MAX_SQUARES));

    (2..=s as u64)
        .fold(1, |acc, _| acc * 2)
}

pub fn total() -> u64 {
    (MIN_SQUARES..=MAX_SQUARES)
        .map(|i| square(i))
        .sum()
}
