use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::delimited,
    IResult,
};

use Node::*;
use Ordering::*;

#[derive(Debug, Clone, PartialEq)]
enum Node {
    List(Vec<Node>),
    Value(u32),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value(l), Value(r)) => l.partial_cmp(r),
            (ln @ List(_), rn @ Value(_)) => ln.partial_cmp(&List(vec![rn.clone()])),
            (ln @ Value(_), rn @ List(_)) => List(vec![ln.clone()]).partial_cmp(rn),
            (List(l), List(r)) => {
                let content_comparison = l
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, ll)| match r.get(i) {
                        Some(rr) => ll.partial_cmp(rr),
                        None => Some(Greater),
                    })
                    .find(|v| *v != Equal);

                if content_comparison == Some(Equal) || content_comparison == None {
                    if l.len() == r.len() {
                        Some(Equal)
                    } else {
                        Some(Less)
                    }
                } else {
                    content_comparison
                }
            }
        }
    }
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    if input.starts_with("[") {
        let (input, vs) =
            delimited(tag("["), separated_list0(tag(","), parse_node), tag("]"))(input)?;
        Ok((input, Node::List(vs)))
    } else {
        let (input, v) = complete::u32(input)?;
        Ok((input, Node::Value(v)))
    }
}

fn parse_node_pair(input: &str) -> IResult<&str, (Node, Node)> {
    let (input, left) = parse_node(input)?;
    let (input, _) = newline(input)?;
    let (input, right) = parse_node(input)?;
    Ok((input, (left, right)))
}

fn parse_node_pairs(input: &str) -> IResult<&str, Vec<(Node, Node)>> {
    let (input, node_pairs) = separated_list1(tag("\n\n"), parse_node_pair)(input)?;
    Ok((input, node_pairs))
}

pub fn process_part1(input: &str) -> String {
    let (_, node_pairs) = parse_node_pairs(input).unwrap();
    node_pairs
        .iter()
        .enumerate()
        .filter_map(
            |(i, (left, right))| {
                if left < right {
                    Some(i + 1)
                } else {
                    None
                }
            },
        )
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut node_pairs) = parse_node_pairs(input).unwrap();

    let tracer1 = Node::List(vec![Node::List(vec![Node::Value(2)])]);
    let tracer2 = Node::List(vec![Node::List(vec![Node::Value(6)])]);
    node_pairs.push((tracer1.clone(), tracer2.clone()));

    let nodes = node_pairs
        .into_iter()
        .flat_map(|(left, right)| [left, right])
        .sorted_by(|left, right| left.partial_cmp(right).unwrap())
        .enumerate();

    let (tracer1_idx, _) = nodes
        .clone()
        .find(|(_, n)| tracer1.partial_cmp(n) == Some(Ordering::Equal))
        .unwrap();

    let (tracer2_idx, _) = nodes
        .clone()
        .find(|(_, n)| tracer2.partial_cmp(n) == Some(Ordering::Equal))
        .unwrap();

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
