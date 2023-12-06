use nom::bytes::complete::tag;
use nom::character::complete::{newline, space1};
use nom::multi::separated_list1;
use nom::IResult;

fn parse_races(input: &str) -> IResult<&str, Vec<(f64, f64)>> {
    let (input, _) = tag("Time: ")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = separated_list1(space1, nom::number::complete::double)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("Distance: ")(input)?;
    let (input, _) = space1(input)?;
    let (input, distances) = separated_list1(space1, nom::number::complete::double)(input)?;
    Ok((input, times.into_iter().zip(distances).collect()))
}

fn parse_single_race(input: &str) -> (f64, f64) {
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

fn ways_to_win_race(total_time: f64, distance_to_beat: f64) -> f64 {
    let d = (total_time * total_time - 4f64 * distance_to_beat).sqrt();
    let mut root2 = (-total_time - d) / -2f64;
    let mut root1 = (-total_time + d) / -2f64;
    if root1.ceil() == root1 {
        root1 += 1f64;
    }
    if root2.floor() == root2 {
        root2 -= 1f64;
    }
    root2.floor() - root1.ceil() + 1f64
}

pub fn process_part1(input: &str) -> String {
    let (_, races) = parse_races(input).unwrap();
    races
        .into_iter()
        .map(|(total_time, distance_to_beat)| {
            ways_to_win_race(total_time, distance_to_beat)
        })
        .product::<f64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (total_time, distance_to_beat) = parse_single_race(input);
    ways_to_win_race(total_time, distance_to_beat).to_string()
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
