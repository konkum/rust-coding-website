pub fn digit_factorials() {
    let limit = 7 * factorial_of_digit(9);

    let mut result = 0;
    for i in 3..limit {
        if sum_of_factorial_of_digits(i) == i {
            result += i;
        }
    }

    println!("{}", result);
}

fn sum_of_factorial_of_digits(mut number: u32) -> u32 {
    let mut sum = 0;
    while number > 0 {
        sum += factorial_of_digit(number % 10);
        number /= 10;
    }
    return sum;
}

fn factorial_of_digit(number: u32) -> u32 {
    match number {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 6,
        4 => 24,
        5 => 120,
        6 => 720,
        7 => 5040,
        8 => 40320,
        9 => 362880,
        _ => 0,
    }
}
