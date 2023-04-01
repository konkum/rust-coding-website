pub fn _10001st_prime() {
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