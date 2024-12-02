use std::fs::read_to_string;

fn main() {
    println!("Day 1, part 1: {}", day1_part1());
    println!("Day 1, part 2: {}", day1_part2());

    println!("Day 2, part 1: {}", day2_part1());
    println!("Day 2, part 2: {}", day2_part2());
}

fn day2_part1() -> i32 {
    let input_name = "inputs/d2_test.txt";
    let input = read_to_string(input_name).unwrap();
    let mut count = 0;

    let valid = |measurements: &[i32]| -> bool {
        let mut i = 1;
        let increasing = measurements[0] < measurements[1];
        while i < measurements.len() {
            let curr = measurements[i];
            let prev = measurements[i - 1];
            let diff = (curr - prev).abs();
            if !(1..=3).contains(&diff) {
                return false;
            }
            if curr < prev && increasing {
                return false;
            }
            if curr > prev && !increasing {
                return false;
            }
            i += 1;
        }
        true
    };

    return input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|split| split.map(|x| x.parse::<i32>().unwrap()))
        .map(|parsed| valid(&parsed.collect::<Vec<_>>()) as i32)
        .sum();
}
fn day2_part2() -> i32 {
    0
}

fn day1_part1() -> i32 {
    let input_name = "inputs/d1_test.txt";
    let input = read_to_string(input_name).unwrap();
    let mut left = vec![];
    let mut right = vec![];
    input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .for_each(|xs| {
            left.push(xs[0].parse::<i32>().unwrap());
            right.push(xs[1].parse::<i32>().unwrap());
        });
    left.sort();
    right.sort();
    left.iter().zip(right).map(|(a, b)| (a - b).abs()).sum()
}

fn day1_part2() -> i32 {
    let input_name = "inputs/d1_test.txt";
    let input = read_to_string(input_name).unwrap();
    let mut left = vec![];
    let mut right = vec![];
    input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .for_each(|xs| {
            left.push(xs[0].parse::<i32>().unwrap());
            right.push(xs[1].parse::<i32>().unwrap());
        });
    let mut score = 0;
    for x in left {
        let mut count = 0;
        for y in &right {
            count += (x == *y) as i32;
        }
        score += count * x;
    }
    score
}
