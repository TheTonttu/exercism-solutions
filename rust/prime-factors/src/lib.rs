pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut remaining = n;
    let mut divisor = 2;

    while divisor <= remaining && remaining > 0 {
        while remaining % divisor == 0 {
            factors.push(divisor);
            remaining /= divisor;
        }
        divisor += 1;
    }
    factors
}
