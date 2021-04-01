
#[test]
fn test_split_zero() {
    assert_eq!(say::split_thousand_to_smaller(0), vec![0, 0, 0]);
}

#[test]
fn test_split_one() {
    assert_eq!(say::split_thousand_to_smaller(1), vec![0, 0, 1]);
}

#[test]
fn test_split_ten() {
    assert_eq!(say::split_thousand_to_smaller(10), vec![0, 0, 10]);
}

#[test]
fn test_split_twenty() {
    assert_eq!(say::split_thousand_to_smaller(20), vec![0, 20, 0]);
}

#[test]
fn test_split_hundred() {
    assert_eq!(say::split_thousand_to_smaller(100), vec![100, 0, 0]);
}

#[test]
fn test_split_hundred_and_eleven() {
    assert_eq!(say::split_thousand_to_smaller(111), vec![100, 0, 11]);
}

#[test]
fn test_split_hundred_and_twenty_one() {
    assert_eq!(say::split_thousand_to_smaller(121), vec![100, 20, 1]);
}