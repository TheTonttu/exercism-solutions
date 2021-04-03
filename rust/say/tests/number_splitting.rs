#[test]
fn test_split_one_thousand_eight_hundred_and_seven() {
    assert_eq!(say::split_number_to_thousands(&1807), vec![1, 807]);
}
