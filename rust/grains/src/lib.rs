const MIN_SQUARES: u32 = 1;
const MAX_SQUARES: u32 = 64;

pub fn square(s: u32) -> u64 {
    assert!(MIN_SQUARES <= s && s <= MAX_SQUARES, "Square must be between {} and {}", MIN_SQUARES, MAX_SQUARES);

    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (MIN_SQUARES..=MAX_SQUARES)
        .map(square)
        .sum()
}
