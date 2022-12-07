use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair, *,
};

fn assignment(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, (start, end)) =
        separated_pair(character::complete::u32, tag("-"), character::complete::u32)(input)?;
    Ok((input, start..=end))
}

fn pair_assignments(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, (first, second)) = separated_pair(assignment, tag(","), assignment)(input)?;
    Ok((input, (first, second)))
}

fn all_assignments(input: &str) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let (input, pairs) = separated_list1(newline, pair_assignments)(input)?;
    Ok((input, pairs))
}

pub fn process_part1(input: String) -> String {
    let (_, assignments) = all_assignments(&input).unwrap();
    assignments
        .iter()
        .filter(|(a, b)| {
            let a_contains_b = b.clone().into_iter().all(|num| a.contains(&num));
            let b_contains_a = a.clone().into_iter().all(|num| b.contains(&num));
            a_contains_b || b_contains_a
        })
        .count()
        .to_string()
}

pub fn process_part2(input: String) -> String {
    let (_, assignments) = all_assignments(&input).unwrap();
    assignments
        .iter()
        .filter(|(a, b)| {
            let a_overlaps_b = b.clone().into_iter().any(|num| a.contains(&num));
            let b_overlaps_a = a.clone().into_iter().any(|num| b.contains(&num));
            a_overlaps_b || b_overlaps_a
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT.to_string()), "2");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT.to_string()), "4");
    }
}
