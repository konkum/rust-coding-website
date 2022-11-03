pub fn multiples_of_3_or_5() {
    let mut sum = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("{}", sum);
}

pub fn even_fibonacci_numbers() {
    let mut a = 1;
    let mut b = 2;
    let mut sum = 2;
    let mut c: i64 = 0;
    while c < 4000000 {
        if c % 2 == 0 {
            sum += c;
        }
        c = a + b;
        a = b;
        b = c;
    }
    println!("{}", sum);
}

pub fn sum_square_difference() {
    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 1..=100 {
        sum1 += i * i;
        sum2 += i;
    }
    println!("{}", sum2 * sum2 - sum1);
}

pub fn smallest_multiple() {
    fn check_divisable(num: i32) -> bool {
        for i in 1..=20 {
            if num % i != 0 {
                return false;
            }
        }
        return true;
    }
    let mut number = 2;
    loop {
        number += 2;
        if check_divisable(number) {
            break;
        } else {
            continue;
        }
    }
    println!("{}", number);
}

pub fn largest_prime_factor() {
    fn is_prime(num: i64) -> bool {
        if num < 2 {
            return false;
        }
        if num % 2 == 0 {
            return num == 2;
        }
        let mut root = (num as f64).sqrt() as i64;
        for i in (3..=root).step_by(2) {
            if num % i == 0 {
                return false;
            }
        }
        return true;
    }
    let num = 600851475143;
    let count = (num as f64).sqrt() as i64 + 1;
    let mut result = 0;
    for i in 2..count {
        if num % i == 0 {
            if is_prime(i) {
                result = i;
            }
        }
    }
    println!("{}", result);
}