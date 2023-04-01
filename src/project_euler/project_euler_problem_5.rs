pub fn smallest_multiple() {
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

fn check_divisable(num: i32) -> bool {
    for i in 1..=20 {
        if num % i != 0 {
            return false;
        }
    }
    return true;
}