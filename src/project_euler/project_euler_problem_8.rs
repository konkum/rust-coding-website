use std::fs;

pub fn largest_product_in_a_series() {
    let input = fs::read_to_string("src/project_euler/input/largest_product_in_a_series.data").unwrap();
    let mut largest = 0;
    let input_bytes = input.as_bytes();
    let mut largest_string: &[u8] = &input_bytes[0..1];
    let span_width = 13;

    for i in 0..(input_bytes.len() - span_width + 1) {
        let mut sum = 1u64;
        for j in 0..span_width {
            sum *= (input_bytes[i + j] - 48) as u64;
        }
        if sum > largest {
            largest = sum;
            largest_string = &input_bytes[i..(i + span_width)];
        }
    }

    println!("Largest: {} is {:?}", largest, largest_string);
}