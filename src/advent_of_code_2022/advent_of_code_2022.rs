pub fn day1() {
    let input = std::fs::read_to_string("src/advent_of_code_2022/input/day1.txt").unwrap();

    let mut elves = Vec::new();
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<usize>().unwrap();
        }
    }

    println!("max = {}", elves.iter().max().unwrap()); // part 1

    elves.sort();
    elves.reverse();
    println!("three max = {}", elves[..3].iter().sum::<usize>()); //part 2
}
