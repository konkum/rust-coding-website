pub fn digit_fifth_powers() {
    let mut result = 0;
    for i in 2..500000 {
        let mut number = i;
        let mut sum = 0;
        loop {
            let digit = number % 10;
            sum += i32::pow(digit, 5);
            number /= 10;

            if number < 1 {
                break;
            }
        }

        if sum == i {
            result += i;
        }
    }

    println!("{result}");
}