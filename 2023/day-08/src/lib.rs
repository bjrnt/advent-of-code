use std::collections::HashMap;

use aoc_utils::lcm;
use itertools::{FoldWhile, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace1, not_line_ending},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Step {
    Left,
    Right,
}

impl TryFrom<char> for Step {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Step::Left),
            'R' => Ok(Step::Right),
            _ => Err(()),
        }
    }
}

fn parse_node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, origin) = terminated(alphanumeric1, tag(" = ("))(input)?;
    let (input, (left, right)) = separated_pair(alphanumeric1, tag(", "), alphanumeric1)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, (origin, (left, right))))
}

fn parse_graph(input: &str) -> IResult<&str, (Vec<Step>, HashMap<&str, (&str, &str)>)> {
    let (input, steps) = not_line_ending(input).map(|(input, steps)| {
        (
            input,
            steps
                .chars()
                .map(|c| c.try_into().unwrap())
                .collect::<Vec<Step>>(),
        )
    })?;
    let (input, _) = multispace1(input)?;
    let (input, nodes) = separated_list1(multispace1, parse_node)(input)?;
    Ok((input, (steps, nodes.into_iter().collect())))
}

pub fn process_part1(input: &str) -> String {
    let (_, (steps, graph)) = parse_graph(input).unwrap();
    steps
        .into_iter()
        .cycle()
        .fold_while(("AAA", 0), |(current_node, total_steps), next_step| {
            if current_node == "ZZZ" {
                FoldWhile::Done((current_node, total_steps))
            } else {
                let (left, right) = graph.get(current_node).unwrap();
                match next_step {
                    Step::Left => FoldWhile::Continue((left, total_steps + 1)),
                    Step::Right => FoldWhile::Continue((right, total_steps + 1)),
                }
            }
        })
        .into_inner()
        .1
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, (steps, graph)) = parse_graph(input).unwrap();
    let starting_nodes: Vec<&&str> = graph.keys().filter(|node| node.ends_with("A")).collect();
    let cycle_lengths: Vec<usize> = starting_nodes
        .into_iter()
        .map(|starting_node| {
            let mut seen_at: HashMap<&str, usize> = HashMap::new();
            steps
                .iter()
                .cycle()
                .fold_while(
                    (starting_node, 0),
                    |(current_node, total_steps), next_step| {
                        let current_node_seen_at =
                            seen_at.entry(&current_node).or_insert(total_steps);
                        if current_node.ends_with("Z") && *current_node_seen_at < total_steps {
                            FoldWhile::Done((current_node, total_steps - *current_node_seen_at))
                        } else {
                            let (left, right) = graph.get(current_node).unwrap();
                            match next_step {
                                Step::Left => FoldWhile::Continue((left, total_steps + 1)),
                                Step::Right => FoldWhile::Continue((right, total_steps + 1)),
                            }
                        }
                    },
                )
                .into_inner()
                .1
        })
        .collect();
    lcm(cycle_lengths.as_slice()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        2
    )]
    #[case(
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        6
    )]
    #[trace]
    fn test_fn(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(process_part1(input).parse::<u32>().unwrap(), expected);
    }

    #[rstest]
    #[case(
        "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)",
        6
    )]
    #[trace]
    fn test_fn2(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(process_part2(input).parse::<u32>().unwrap(), expected);
    }
}
