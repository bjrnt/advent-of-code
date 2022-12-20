use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

use Packet::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Value(u32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (List(l), List(r)) => l.cmp(r),
            (Value(l), Value(r)) => l.cmp(r),
            (l @ Value(_), List(r)) => vec![l.clone()].cmp(r),
            (List(l), r @ Value(_)) => l.cmp(&vec![r.clone()]),
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")).map(List),
        complete::u32.map(Value),
    ))(input)
}

fn parse_packet_pairs(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(parse_packet, newline, parse_packet),
    )(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, packet_pairs) = parse_packet_pairs(input).unwrap();
    packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (left, right))| (left < right).then_some(i + 1))
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, packet_pairs) = parse_packet_pairs(input).unwrap();

    let tracer1 = List(vec![List(vec![Value(2)])]);
    let tracer2 = List(vec![List(vec![Value(6)])]);

    let mut packets = packet_pairs
        .into_iter()
        .chain([(tracer1.clone(), tracer2.clone())])
        .flat_map(|(l, r)| [l, r])
        .sorted();

    let tracer1_idx = packets.clone().position(|p| tracer1 == p).unwrap();
    let tracer2_idx = packets.position(|p| tracer2 == p).unwrap();

    ((tracer1_idx + 1) * (tracer2_idx + 1)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "13");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "140");
    }
}
