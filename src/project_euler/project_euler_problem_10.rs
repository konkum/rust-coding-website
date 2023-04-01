pub fn summation_of_primes() {
    let mut sum = 2;
    for i in (3..2000000).step_by(2) {
        if is_prime(i) {
            sum += i;
        }
    }
    println!("{}", sum);
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