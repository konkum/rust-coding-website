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