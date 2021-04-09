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

    let aliquot_sum = calculate_aliquot_sum(num);
    println!("{:?}", aliquot_sum);

    let classification = match aliquot_sum.cmp(&num) {
        Ordering::Less => Classification::Deficient,
        Ordering::Equal => Classification::Perfect,
        Ordering::Greater => Classification::Abundant,
    };

    Some(classification)
}

fn calculate_aliquot_sum(num: u64) -> u64 {
    let num_sqrt = (num as f64).sqrt() as u64;
    // Sum of all the possible divisors
    let divisors_sum: u64 = (1..=num_sqrt)
        .filter(|n| num % n == 0)
        .map(|n| {
            let quotient = num / n;
            if quotient == n {
                n
            } else {
                n + quotient
            }
        })
        .sum();
    // Sum of all the possible divisors - the number =  the sum of proper divisors
    divisors_sum - num
}
