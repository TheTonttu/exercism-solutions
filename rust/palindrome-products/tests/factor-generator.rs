
use palindrome_products::{FactorGenerator};

#[test]
fn test_gen_starts_from_begin() {
    let mut gen = FactorGenerator::new(1, 10);
    assert_eq!(gen.next(), Some((1, 1)));
}

#[test]
fn test_gen_factors_grow() {
    let mut gen = FactorGenerator::new(1, 10);
    gen.next();
    assert_eq!(gen.next(), Some((1, 2)));
}

#[test]
fn test_gen_factors_grow_more() {
    let mut gen = FactorGenerator::new(1, 10);
    gen.next();
    gen.next();
    assert_eq!(gen.next(), Some((1, 3)));
}

#[test]
fn test_gen_factors_grow_even_more() {
    let mut gen = FactorGenerator::new(1, 10);
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    assert_eq!(gen.next(), Some((2, 1)));
}

#[test]
fn test_gen_factors_last() {
    let mut gen = FactorGenerator::new(1, 2);
    let (i, last) = (1..100).map(|i| (i, gen.next()))
        .take_while(|(_, factors)| factors.is_some())
        .last().unwrap();
    assert_eq!(i, 4);
    assert_eq!(last, Some((2,2)));
}

#[test]
fn test_gen_factors_end() {
    let mut gen = FactorGenerator::new(1, 2);
    gen.next();
    gen.next();
    gen.next();
    assert_eq!(gen.next(), Some((2,2)));
    gen.next();
    assert_eq!(gen.next(), None);
}

#[test]
fn test_gen_rev_starts_from_begin() {
    let mut gen = FactorGenerator::new(10, 1);
    assert_eq!(gen.next(), Some((10, 10)));
}

#[test]
fn test_gen_rev_shrink() {
    let mut gen = FactorGenerator::new(10, 1);
    gen.next();
    assert_eq!(gen.next(), Some((10, 9)));
}

#[test]
fn test_gen_rev_shrink_more() {
    let mut gen = FactorGenerator::new(10, 1);
    gen.next();
    gen.next();
    assert_eq!(gen.next(), Some((10, 8)));
}

#[test]
fn test_gen_factors_shrink_even_more() {
    let mut gen = FactorGenerator::new(10, 1);
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    gen.next();
    assert_eq!(gen.next(), Some((9, 10)));
}

#[test]
fn test_gen_rev_factors_end() {
    let mut gen = FactorGenerator::new(2, 1);
    gen.next();
    gen.next();
    gen.next();
    assert_eq!(gen.next(), Some((1,1)));
    gen.next();
    assert_eq!(gen.next(), None);
}

#[test]
fn test_gen_rev_factors_last() {
    let mut gen = FactorGenerator::new(2, 1);
    let (i, last) = (1..100).map(|i| (i, gen.next()))
                            .take_while(|(_, factors)| factors.is_some())
                            .last().unwrap();
    assert_eq!(i, 4);
    assert_eq!(last, Some((1,1)));
}