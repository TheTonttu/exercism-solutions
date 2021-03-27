//! This test suite was generated by the rust exercise tool, which can be found at
//! https://github.com/exercism/rust/tree/main/util/exercise

use palindrome_products::{palindrome_products, Palindrome};

/// Process a single test case for the property `smallest`
///
/// All cases for the `smallest` property are implemented
/// in terms of this function.
fn process_smallest_case(input: (u64, u64), expected: Option<Palindrome>) {
    let min = palindrome_products(input.0, input.1).map(|(min, _)| min);
    assert_eq!(min, expected);
}

/// Process a single test case for the property `largest`
///
/// All cases for the `largest` property are implemented
/// in terms of this function.
fn process_largest_case(input: (u64, u64), expected: Option<Palindrome>) {
    let max = palindrome_products(input.0, input.1).map(|(_, max)| max);
    assert_eq!(max, expected);
}

#[test]
/// finds the smallest palindrome from single digit factors
fn test_finds_the_smallest_palindrome_from_single_digit_factors() {
    process_smallest_case((1, 9), Some(Palindrome::new(1, 1)));
}

#[test]
/// finds the largest palindrome from single digit factors
fn test_finds_the_largest_palindrome_from_single_digit_factors() {
    let mut expect = Palindrome::new(1, 9);
    expect.insert(3, 3);
    process_largest_case((1, 9), Some(expect));
}

#[test]
/// find the smallest palindrome from double digit factors
fn test_find_the_smallest_palindrome_from_double_digit_factors() {
    process_smallest_case((10, 99), Some(Palindrome::new(11, 11)));
}

#[test]
/// find the largest palindrome from double digit factors
fn test_find_the_largest_palindrome_from_double_digit_factors() {
    process_largest_case((10, 99), Some(Palindrome::new(91, 99)));
}

#[test]
/// find smallest palindrome from triple digit factors
fn test_find_smallest_palindrome_from_triple_digit_factors() {
    process_smallest_case((100, 999), Some(Palindrome::new(101, 101)));
}

#[test]
/// find the largest palindrome from triple digit factors
fn test_find_the_largest_palindrome_from_triple_digit_factors() {
    process_largest_case((100, 999), Some(Palindrome::new(913, 993)));
}

#[test]
#[ignore]
// TODO: Optimize
/// find smallest palindrome from four digit factors
fn test_find_smallest_palindrome_from_four_digit_factors() {
    process_smallest_case((1000, 9999), Some(Palindrome::new(1001, 1001)));
}

#[test]
#[ignore]
// TODO: Optimize
/// find the largest palindrome from four digit factors
fn test_find_the_largest_palindrome_from_four_digit_factors() {
    process_largest_case((1000, 9999), Some(Palindrome::new(9901, 9999)));
}

#[test]
/// empty result for smallest if no palindrome in the range
fn test_empty_result_for_smallest_if_no_palindrome_in_the_range() {
    process_smallest_case((1002, 1003), None);
}

#[test]
/// empty result for largest if no palindrome in the range
fn test_empty_result_for_largest_if_no_palindrome_in_the_range() {
    process_largest_case((15, 15), None);
}

#[test]
/// error result for smallest if min is more than max
fn test_error_result_for_smallest_if_min_is_more_than_max() {
    process_smallest_case((10000, 1), None);
}

#[test]
/// error result for largest if min is more than max
fn test_error_result_for_largest_if_min_is_more_than_max() {
    process_largest_case((2, 1), None);
}
