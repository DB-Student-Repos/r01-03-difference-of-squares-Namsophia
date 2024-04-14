pub fn square_of_sum(n: u32) -> u32 {
    //unimplemented!("square of sum of 1...{n}")
    let mut numbers = 0;
    for i in 0..n + 1 {
        numbers += i;
    }
    return numbers.pow(2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    //unimplemented!("sum of squares of 1...{n}")
        let mut numbers = vec![];
    for i in 0..n + 1 {
        numbers.push(i);
    }

    let sum: u32 = numbers.iter().fold(0, |acc, number| acc + number.pow(2));
    return sum;
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n);
    // unimplemented!("difference between square of sum of 1...{n} and sum of squares of 1...{n}");
}
