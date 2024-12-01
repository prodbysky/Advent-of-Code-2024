use std::fs::read_to_string;

fn day1_part1() -> i32 {
    let input_name = "inputs/d1p1_test.txt";
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
    let input_name = "inputs/d1p1_test.txt";
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

fn main() {
    println!("Day 1, part 1: {}", day1_part1());
    println!("Day 1, part 2: {}", day1_part2());
}
