use day_aoc::process_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{} should be 5904", process_part1(file.as_str()));
}
