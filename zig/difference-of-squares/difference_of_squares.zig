pub fn squareOfSum(number: usize) usize {
    var sum: usize = 0;
    for (1..number + 1) |n| {
        sum += n;
    }
    return sum * sum;
}

pub fn sumOfSquares(number: usize) usize {
    var sum: usize = 0;
    for (1..number + 1) |n| {
        sum += n * n;
    }
    return sum;
}

pub fn differenceOfSquares(number: usize) usize {
    return squareOfSum(number) - sumOfSquares(number);
}
