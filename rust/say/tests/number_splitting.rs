#[test]
fn test_split_zero() {
    assert_eq!(say::split_thousand_to_smaller(&0), vec![0, 0, 0]);
}

#[test]
fn test_split_one() {
    assert_eq!(say::split_thousand_to_smaller(&1), vec![0, 0, 1]);
}

#[test]
fn test_split_ten() {
    assert_eq!(say::split_thousand_to_smaller(&10), vec![0, 0, 10]);
}

#[test]
fn test_split_twenty() {
    assert_eq!(say::split_thousand_to_smaller(&20), vec![0, 20, 0]);
}

#[test]
fn test_split_fifty() {
    assert_eq!(say::split_thousand_to_smaller(&50), vec![0, 50, 0]);
}

#[test]
fn test_split_hundred() {
    assert_eq!(say::split_thousand_to_smaller(&100), vec![100, 0, 0]);
}

#[test]
fn test_split_hundred_and_eleven() {
    assert_eq!(say::split_thousand_to_smaller(&111), vec![100, 0, 11]);
}

#[test]
fn test_split_six_hundred_and_fifteen() {
    assert_eq!(say::split_thousand_to_smaller(&615), vec![600, 0, 15]);
}

#[test]
fn test_split_eight_hundred_and_seven() {
    assert_eq!(say::split_thousand_to_smaller(&807), vec![800, 0, 7]);
}

#[test]
fn test_split_hundred_and_twenty_one() {
    assert_eq!(say::split_thousand_to_smaller(&121), vec![100, 20, 1]);
}

#[test]
fn test_split_one_thousand_eight_hundred_and_seven() {
    assert_eq!(say::split_number_to_thousands(&1807), vec![1, 807]);
}
