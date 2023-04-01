use std::arch::asm;
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

pub fn largest_product_in_a_series() {
    let input = fs::read_to_string("src/project_euler/input/largest_product_in_a_series.data").unwrap();
    let mut largest = 0;
    let input_bytes = input.as_bytes();
    let mut largest_string: &[u8] = &input_bytes[0..1];
    let span_width = 13;

    for i in 0..(input_bytes.len() - span_width + 1) {
        let mut sum = 1u64;
        for j in 0..span_width {
            sum *= (input_bytes[i + j] - 48) as u64;
        }
        if sum > largest {
            largest = sum;
            largest_string = &input_bytes[i..(i + span_width)];
        }
    }

    println!("Largest: {} is {:?}", largest, largest_string);
}

pub fn largest_product_in_a_grid() {
    let input = fs::read_to_string("src/project_euler/input/largest_product_in_a_grid.data").unwrap();
    let input_split = input.split_whitespace();
    let input_as_num: Vec<i32> = input_split.map(|x|
        match i32::from_str_radix(x, 10) {
            Ok(v) => v,
            Err(u) => {
                print!("Garbage in input: {}", u);
                0
            }
        }
    ).collect();

    const GRID_SIZE: usize = 20;
    const ANSWER_LENGTH: usize = 4;

    let element_at = |x: usize, y: usize| {
        return input_as_num[y * GRID_SIZE + x];
    };

    let stride_sum = |x: usize, y: usize, x_stride: i32, y_stride: i32| {
        let mut answer = 1;
        for current in 0..ANSWER_LENGTH {
            answer *= element_at(((x as i32) + x_stride * (current as i32)) as usize,
                                 ((y as i32) + y_stride * (current as i32)) as usize);
        }
        return answer;
    };

    let mut greatest_answer = 0;

    for y in 0..(GRID_SIZE - ANSWER_LENGTH + 1) {
        for x in 0..GRID_SIZE {
            //DOWN
            {
                let answer = stride_sum(x, y, 0, 1);
                if answer > greatest_answer { greatest_answer = answer; }
            }
            //DOWN+LEFT
            if x >= (ANSWER_LENGTH - 1) {
                let answer = stride_sum(x, y, -1, 1);
                if answer > greatest_answer { greatest_answer = answer; }
            }
            //DOWN+RIGHT and RIGHT
            if x <= (GRID_SIZE - ANSWER_LENGTH) {
                let down_right_answer = stride_sum(x, y, 1, 1);
                if down_right_answer > greatest_answer { greatest_answer = down_right_answer; }

                let right_answer = stride_sum(x, y, 1, 0);
                if right_answer > greatest_answer { greatest_answer = right_answer; }
            }
        }
    }

    println!("{}", greatest_answer);
}