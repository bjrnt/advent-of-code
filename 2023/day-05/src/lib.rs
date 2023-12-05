use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, newline, not_line_ending, space0, space1},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("seeds: ")(input)?;
    separated_list1(space1, nom::character::complete::u64)(input)
}

fn parse_seed_ranges(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, ranges) = separated_list1(
        space1,
        separated_pair(
            nom::character::complete::u64,
            space1,
            nom::character::complete::u64,
        ),
    )(input)?;
    let seeds = ranges
        .into_iter()
        .flat_map(|(range_start, range_length)| (range_start..range_start + range_length))
        .collect();
    Ok((input, seeds))
}

fn parse_range(input: &str) -> IResult<&str, (Range<u64>, u64)> {
    let (input, destination_range_start) =
        terminated(nom::character::complete::u64, space1)(input)?;
    let (input, source_range_start) = terminated(nom::character::complete::u64, space1)(input)?;
    let (input, range_length) = terminated(nom::character::complete::u64, space0)(input)?;
    Ok((
        input,
        (
            source_range_start..source_range_start + range_length,
            destination_range_start,
        ),
    ))
}

fn parse_map(input: &str) -> IResult<&str, Vec<(Range<u64>, u64)>> {
    let (input, _) = multispace0(input)?;
    let (input, _) = not_line_ending(input)?;
    let (input, _) = multispace0(input)?;
    separated_list1(newline, parse_range)(input)
}

fn lookup_in_map(map: &Vec<(Range<u64>, u64)>, val: &u64) -> u64 {
    let Some((source_range, destination_range_start)) = map.iter().find(|(r, _)| r.contains(val))
    else {
        return *val;
    };
    destination_range_start + val - source_range.start
}

pub fn process_part1(input: &str) -> String {
    let (input, seeds) = parse_seeds(input).unwrap();
    let (input, seed_to_soil_map) = parse_map(input).unwrap();
    let (input, soil_to_fertilizer_map) = parse_map(input).unwrap();
    let (input, fertilizer_to_water_map) = parse_map(input).unwrap();
    let (input, water_to_light_map) = parse_map(input).unwrap();
    let (input, light_to_temperature_map) = parse_map(input).unwrap();
    let (input, temperature_to_humidity_map) = parse_map(input).unwrap();
    let (_, humidity_to_location_map) = parse_map(input).unwrap();
    seeds
        .iter()
        .map(|v| lookup_in_map(&seed_to_soil_map, v))
        .map(|v| lookup_in_map(&soil_to_fertilizer_map, &v))
        .map(|v| lookup_in_map(&fertilizer_to_water_map, &v))
        .map(|v| lookup_in_map(&water_to_light_map, &v))
        .map(|v| lookup_in_map(&light_to_temperature_map, &v))
        .map(|v| lookup_in_map(&temperature_to_humidity_map, &v))
        .map(|v| lookup_in_map(&humidity_to_location_map, &v))
        .min()
        .unwrap()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, seeds) = parse_seed_ranges(input).unwrap();
    let (input, seed_to_soil_map) = parse_map(input).unwrap();
    let (input, soil_to_fertilizer_map) = parse_map(input).unwrap();
    let (input, fertilizer_to_water_map) = parse_map(input).unwrap();
    let (input, water_to_light_map) = parse_map(input).unwrap();
    let (input, light_to_temperature_map) = parse_map(input).unwrap();
    let (input, temperature_to_humidity_map) = parse_map(input).unwrap();
    let (_, humidity_to_location_map) = parse_map(input).unwrap();
    seeds
        .into_par_iter()
        .map(|v| lookup_in_map(&seed_to_soil_map, &v))
        .map(|v| lookup_in_map(&soil_to_fertilizer_map, &v))
        .map(|v| lookup_in_map(&fertilizer_to_water_map, &v))
        .map(|v| lookup_in_map(&water_to_light_map, &v))
        .map(|v| lookup_in_map(&light_to_temperature_map, &v))
        .map(|v| lookup_in_map(&temperature_to_humidity_map, &v))
        .map(|v| lookup_in_map(&humidity_to_location_map, &v))
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "35");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "46");
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
