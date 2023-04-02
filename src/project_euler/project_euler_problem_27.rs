pub fn quadratic_primes() {
    let mut best = (0, 0);
    let mut best_count = 0;
    let bs = (0..1000).filter(|&b| is_prime(b)).collect::<Vec<_>>();
    for a in -999..1000 {
        for &b in &bs {
            let count = (0..)
                .take_while(|n| is_prime(n * n + a * n + b))
                .map(|_| 1)
                .sum::<i64>();
            if count > best_count {
                best = (a, b);
                best_count = count;
            }
        }
    }

    println!("{}", best.0 * best.1);
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