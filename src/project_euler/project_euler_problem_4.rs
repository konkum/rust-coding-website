pub fn largest_palindrome_product() {
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