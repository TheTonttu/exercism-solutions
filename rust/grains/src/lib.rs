const TOTAL_SQUARES: u32 = 64;

pub fn square(s: u32) -> u64 {
    assert!(1 <= s && s <= 64, "Square must be between 1 and 64");

    (2..=s as u64)
        .fold(1, |acc, _| acc * 2)
}

pub fn total() -> u64 {
    (1..=TOTAL_SQUARES)
        .map(|i| square(i))
        .sum()
}
