use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 {
        return None;
    }

    let aliquot_sum: u64 = (1..num).filter(|n| num % n == 0).sum();

    println!("{:?}", aliquot_sum);

    let classification = match aliquot_sum.cmp(&num) {
        Ordering::Less => Classification::Deficient,
        Ordering::Equal => Classification::Perfect,
        Ordering::Greater => Classification::Abundant,
    };

    Some(classification)
}
