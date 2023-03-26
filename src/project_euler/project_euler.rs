use std::fs;

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
        let root = (num as f64).sqrt() as i64;
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

pub fn largest_palindrome_product() {
    fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut reverse = 0;
        let mut temp = x;
        while temp != 0 {
            reverse = (reverse * 10) + (temp % 10);
            temp = temp / 10;
        }
        return reverse == x;
    }
    let mut palindrome = 0;
    'outer: for i in (900..1000).rev() {
        for j in (900..i).rev() {
            palindrome = i * j;
            if is_palindrome(palindrome) {
                break 'outer;
            }
        }
    }
    println!("{}", palindrome);
}

pub fn _10001st_prime() {
    fn is_prime(num: i64) -> bool {
        if num < 2 {
            return false;
        }
        if num % 2 == 0 {
            return num == 2;
        }
        let root = (num as f64).sqrt() as i64;
        for i in (3..=root).step_by(2) {
            if num % i == 0 {
                return false;
            }
        }
        return true;
    }
    let mut prime = 3;
    let mut i = 2;
    loop {
        prime += 2;
        if is_prime(prime) {
            i += 1;
            if i == 10001 {
                break;
            }
        }
    }
    println!("{}", prime);
}

pub fn summation_of_primes() {
    fn is_prime(num: i64) -> bool {
        if num < 2 {
            return false;
        }
        if num % 2 == 0 {
            return num == 2;
        }
        let root = (num as f64).sqrt() as i64;
        for i in (3..=root).step_by(2) {
            if num % i == 0 {
                return false;
            }
        }
        return true;
    }

    let mut sum = 2;
    for i in (3..2000000).step_by(2) {
        if is_prime(i) {
            sum += i;
        }
    }
    println!("{}", sum);
}

pub fn power_digit_sum() {
    let mut decimal = vec![1];
    for _ in 0..1000 {
        let mut carry = 0;
        for i in 0..decimal.len() {
            let mut digit = decimal[i];
            digit = 2 * digit + carry;
            carry = digit / 10;
            decimal[i] = digit % 10;
        }
        if carry > 0 {
            decimal.push(carry);
        }
    }
    println!("{}", decimal.iter().sum::<u64>());
}

pub fn longest_collatz_sequence() {
    let mut result = 0;
    let mut count_result = 0;
    for i in 2..1000000 {
        let mut num: i64 = i;
        let mut count = 0;
        loop {
            if num == 1 {
                break;
            }
            if num % 2 == 0 {
                count += 1;
                num /= 2;
                continue;
            } else {
                count += 2;
                num = (3 * num + 1) / 2;
                continue;
            }
        }
        if count_result < count {
            result = i;
            count_result = count;
        }
    }
    println!("number {result} : count {count_result}");
}

pub fn large_sum() {
    let mut sum: f64 = 0_f64;
    let content =
        fs::read_to_string("src/project_euler/input/large_sum.data")
            .unwrap();
    let mut x = 0;
    for line in content.lines() {
        let num: f64 = line.parse().unwrap();
        sum = sum + num;
        x = x + 1;
        if x > 98 {
            break;
        }
    }
    println!("{}", sum)
}
