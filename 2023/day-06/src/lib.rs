use nom::bytes::complete::tag;
use nom::character::complete::{newline, space1};
use nom::multi::separated_list1;
use nom::IResult;

fn parse_races(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (input, _) = tag("Time: ")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = separated_list1(space1, nom::character::complete::u64)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("Distance: ")(input)?;
    let (input, _) = space1(input)?;
    let (input, distances) = separated_list1(space1, nom::character::complete::u64)(input)?;
    Ok((input, times.into_iter().zip(distances).collect()))
}

fn parse_single_race(input: &str) -> (u64, u64) {
    let numbers_in_lines = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
        })
        .collect::<Vec<String>>();
    (
        numbers_in_lines[0].parse().unwrap(),
        numbers_in_lines[1].parse().unwrap(),
    )
}

fn calculate_distance(total_time: u64, charge_for: u64) -> u64 {
    (total_time - charge_for) * charge_for
}

pub fn process_part1(input: &str) -> String {
    let (_, races) = parse_races(input).unwrap();
    races
        .into_iter()
        .map(|(total_time, distance_to_beat)| {
            (1..total_time)
                .filter(|charge_for| calculate_distance(total_time, *charge_for) > distance_to_beat)
                .count()
        })
        .product::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (total_time, distance_to_beat) = parse_single_race(input);
    (1..total_time)
        .filter(|charge_for| calculate_distance(total_time, *charge_for) > distance_to_beat)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "288");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "71503");
    }
}
