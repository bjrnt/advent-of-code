use day_aoc::process_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part2(file.as_str(), 4_000_000));
}
