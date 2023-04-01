pub fn largest_prime_factor() {
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