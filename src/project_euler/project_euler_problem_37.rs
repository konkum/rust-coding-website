use std::arch::asm;
use std::iter::successors;

pub fn truncatable_primes() {
    let mut result = 0;
    for i in 10..500000 {
        if is_prime(i) {
            let mut is_left_prime: bool = false;
            let mut is_right_prime: bool = false;
            let mut left_number = i;
            let mut right_number = i;
            loop {
                left_number /= i64::pow(10, (length(left_number as u32, 10) - 1));
                if !is_prime(left_number) {
                    break;
                }

                if left_number < 10 {
                    is_left_prime = true;
                    break;
                }
            }

            loop {
                right_number /= 10;
                if !is_prime(right_number) {
                    break;
                }

                if right_number < 10 {
                    is_right_prime = true;
                    break;
                }
            }

            if is_right_prime && is_left_prime {
                println!("{}", i);
                result += i;
            }
        }
    }
    println!("{}", result);
}

fn length(n: u32, base: u32) -> u32 {
    let mut power = base;
    let mut count = 1;
    while n >= power {
        count += 1;
        if let Some(new_power) = power.checked_mul(base) {
            power = new_power;
        } else {
            break;
        }
    }
    count
}

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