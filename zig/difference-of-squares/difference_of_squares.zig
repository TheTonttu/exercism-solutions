pub fn squareOfSum(number: usize) usize {
    var sum: usize = 0;
    for (0..number + 1) |i| {
        sum += i;
    }
    return sum * sum;
}

pub fn sumOfSquares(number: usize) usize {
    var sum: usize = 0;
    for (0..number + 1) |i| {
        sum += i * i;
    }
    return sum;
}

pub fn differenceOfSquares(number: usize) usize {
    return squareOfSum(number) - sumOfSquares(number);
}
