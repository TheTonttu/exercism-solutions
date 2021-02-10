pub fn square_of_sum(n: u32) -> u32 {
    let sum = n*(n+1)/2;
    sum*sum

    // (1..=n)
    //     .sum::<u32>()
    //     .pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // Gauss ❤
    (n*(n+1)*(2*n+1))/6

    // (1..=n)
    //     .map(|i| i.pow(2))
    //     .sum::<u32>()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
