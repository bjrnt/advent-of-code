use std::collections::VecDeque;

use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(newline, complete::i64)(input)
}
pub fn process_part1(input: &str) -> String {
    let (_, numbers) = parse_input(input).unwrap();
    let mut list = VecDeque::from_iter(numbers.into_iter().enumerate());
    let original_list = list.clone();
    for (idx, num) in original_list.iter() {
        let current_index = list
            .iter()
            .position(|(i, n)| *n == *num && *i == *idx)
            .unwrap();
        list.remove(current_index);
        list.insert(
            (current_index as i64 + num).rem_euclid(list.len() as i64) as usize,
            (*idx, *num),
        );
    }
    let zero_pos = list.iter().position(|(_, n)| *n == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|pos| list.get((zero_pos + pos) % list.len()).unwrap().1)
        .sum::<i64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, numbers) = parse_input(input).unwrap();
    let mut list = VecDeque::from_iter(numbers.into_iter().map(|n| n * 811589153).enumerate());
    let original_list = list.clone();
    for _ in 0..10 {
        for (idx, num) in original_list.iter() {
            let current_index = list
                .iter()
                .position(|(i, n)| *n == *num && *i == *idx)
                .unwrap();
            list.remove(current_index);
            list.insert(
                (current_index as i64 + num).rem_euclid(list.len() as i64) as usize,
                (*idx, *num),
            );
        }
    }
    let zero_pos = list.iter().position(|(_, n)| *n == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|pos| list.get((zero_pos + pos) % list.len()).unwrap().1)
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "3");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "1623178306");
    }
}
