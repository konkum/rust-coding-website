use std::fs;

pub fn large_sum() {
    let mut sum: f64 = 0_f64;
    let content =
        fs::read_to_string("src/project_euler/input/large_sum.data")
            .unwrap();
    let mut x = 0;
    for line in content.lines() {
        let num: f64 = line.parse().unwrap();
        sum = sum + num;
        x = x + 1;
        if x > 98 {
            break;
        }
    }
    println!("{}", sum)
}
