#[test]
fn test_split_zero() {
    assert_eq!(say::number_to_digits(&0), vec![0]);
}

#[test]
fn test_split_one() {
    assert_eq!(say::number_to_digits(&1), vec![1]);
}

#[test]
fn test_split_ten() {
    assert_eq!(say::number_to_digits(&10), vec![1, 0]);
}

#[test]
fn test_split_twenty() {
    assert_eq!(say::number_to_digits(&20), vec![2, 0]);
}

#[test]
fn test_split_fifty() {
    assert_eq!(say::number_to_digits(&50), vec![5, 0]);
}

#[test]
fn test_split_hundred() {
    assert_eq!(say::number_to_digits(&100), vec![1, 0, 0]);
}

#[test]
fn test_split_hundred_and_eleven() {
    assert_eq!(say::number_to_digits(&111), vec![1, 1, 1]);
}

#[test]
fn test_split_six_hundred_and_fifteen() {
    assert_eq!(say::number_to_digits(&615), vec![6, 1, 5]);
}

#[test]
fn test_split_eight_hundred_and_seven() {
    assert_eq!(say::number_to_digits(&807), vec![8, 0, 7]);
}

#[test]
fn test_split_hundred_and_twenty_one() {
    assert_eq!(say::number_to_digits(&121), vec![1, 2, 1]);
}

#[test]
fn test_split_one_thousand_eight_hundred_and_seven() {
    assert_eq!(say::split_number_to_thousands(&1807), vec![1, 807]);
}
