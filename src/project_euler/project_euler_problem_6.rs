pub fn sum_square_difference() {
    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 1..=100 {
        sum1 += i * i;
        sum2 += i;
    }
    println!("{}", sum2 * sum2 - sum1);
}