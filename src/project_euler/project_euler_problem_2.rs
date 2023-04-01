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