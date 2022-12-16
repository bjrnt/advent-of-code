use std::{collections::HashMap, hash::Hash, ops::RangeInclusive};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use rayon::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

fn position(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("x="), nom::character::complete::i32),
        tag(", "),
        preceded(tag("y="), nom::character::complete::i32),
    )(input)
}

fn parse_sensor(input: &str) -> IResult<&str, (Point, Point)> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, (sx, sy)) = position(input)?;
    let (input, (bx, by)) = preceded(tag(": closest beacon is at "), position)(input)?;
    Ok((input, (Point { x: sx, y: sy }, Point { x: bx, y: by })))
}

fn parse_sensors_and_beacons(input: &str) -> IResult<&str, Vec<(Point, Point)>> {
    separated_list1(newline, parse_sensor)(input)
}

fn distance(Point { x: x1, y: y1 }: &Point, Point { x: x2, y: y2 }: &Point) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn process_part1(input: &str, y_pos: i32) -> String {
    let (_, sensors_and_beacons) = parse_sensors_and_beacons(input).unwrap();
    let (sensors, beacons): (Vec<_>, Vec<_>) = sensors_and_beacons.into_iter().unzip();

    let sensor_to_distance: HashMap<&Point, i32> = sensors
        .iter()
        .zip(beacons.iter())
        .map(|(sensor, beacon)| (sensor, distance(sensor, beacon)))
        .collect();

    sensor_to_distance
        .iter()
        .filter(|(sensor, distance)| {
            let sensor_y_range = (sensor.y - **distance)..(sensor.y + **distance);
            sensor_y_range.contains(&y_pos)
        })
        .flat_map(|(sensor, max_distance)| {
            let distance_to_line = (sensor.y - y_pos).abs();
            let max_distance_on_line = max_distance - distance_to_line;
            (sensor.x - max_distance_on_line)..=sensor.x + max_distance_on_line
        })
        .unique()
        .filter(|x| !beacons.contains(&Point { x: *x, y: y_pos }))
        .count()
        .to_string()
}

pub fn process_part2(input: &str, maximum: i32) -> String {
    let (_, sensors_and_beacons) = parse_sensors_and_beacons(input).unwrap();
    let (sensors, beacons): (Vec<_>, Vec<_>) = sensors_and_beacons.into_iter().unzip();

    let sensor_to_distance: HashMap<&Point, i32> = sensors
        .iter()
        .zip(beacons.iter())
        .map(|(sensor, beacon)| (sensor, distance(sensor, beacon)))
        .collect();

    const STEP_SIZE: i32 = 1000;

    let (x, y) = (0..=maximum)
        .step_by(STEP_SIZE as usize)
        .par_bridge()
        .find_map_any(|y| {
            (y..y + STEP_SIZE).find_map(|yy| {
                let ranges: Vec<RangeInclusive<i32>> = sensor_to_distance
                    .iter()
                    .filter_map(move |(sensor, max_distance)| {
                        let distance_to_line = (sensor.y - yy).abs();
                        if distance_to_line >= *max_distance {
                            None
                        } else {
                            let max_distance_on_line = *max_distance - distance_to_line;
                            Some(
                                (sensor.x - max_distance_on_line).max(0)
                                    ..=(sensor.x + max_distance_on_line.min(maximum)),
                            )
                        }
                    })
                    .sorted_by_key(|range| (*range.start(), *range.end()))
                    .collect_vec();

                let mut maximum_range = 0..=0;

                for range in ranges.into_iter() {
                    // extend the maximum range if the ranges overlap, or touch with no space in between
                    if maximum_range.end() + 1 >= *range.start() {
                        maximum_range =
                            *maximum_range.start()..=(*maximum_range.end().max(range.end()));
                    } else {
                        return Some((maximum_range.end() + 1, yy));
                    }
                }
                return None;
            })
        })
        .unwrap();

    (x as i64 * 4_000_000 + y as i64).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT, 10), "26");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT, 20), "56000011");
    }
}
