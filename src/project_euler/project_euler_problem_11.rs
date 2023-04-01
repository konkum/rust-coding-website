use std::fs;

pub fn largest_product_in_a_grid() {
    let input = fs::read_to_string("src/project_euler/input/largest_product_in_a_grid.data").unwrap();
    let input_split = input.split_whitespace();
    let input_as_num: Vec<i32> = input_split.map(|x|
        match i32::from_str_radix(x, 10) {
            Ok(v) => v,
            Err(u) => {
                print!("Garbage in input: {}", u);
                0
            }
        }
    ).collect();

    const GRID_SIZE: usize = 20;
    const ANSWER_LENGTH: usize = 4;

    let element_at = |x: usize, y: usize| {
        return input_as_num[y * GRID_SIZE + x];
    };

    let stride_sum = |x: usize, y: usize, x_stride: i32, y_stride: i32| {
        let mut answer = 1;
        for current in 0..ANSWER_LENGTH {
            answer *= element_at(((x as i32) + x_stride * (current as i32)) as usize,
                                 ((y as i32) + y_stride * (current as i32)) as usize);
        }
        return answer;
    };

    let mut greatest_answer = 0;

    for y in 0..(GRID_SIZE - ANSWER_LENGTH + 1) {
        for x in 0..GRID_SIZE {
            //DOWN
            {
                let answer = stride_sum(x, y, 0, 1);
                if answer > greatest_answer { greatest_answer = answer; }
            }
            //DOWN+LEFT
            if x >= (ANSWER_LENGTH - 1) {
                let answer = stride_sum(x, y, -1, 1);
                if answer > greatest_answer { greatest_answer = answer; }
            }
            //DOWN+RIGHT and RIGHT
            if x <= (GRID_SIZE - ANSWER_LENGTH) {
                let down_right_answer = stride_sum(x, y, 1, 1);
                if down_right_answer > greatest_answer { greatest_answer = down_right_answer; }

                let right_answer = stride_sum(x, y, 1, 0);
                if right_answer > greatest_answer { greatest_answer = right_answer; }
            }
        }
    }

    println!("{}", greatest_answer);
}