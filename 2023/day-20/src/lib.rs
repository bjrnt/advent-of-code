use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

const HIGH: u8 = 1u8;
const LOW: u8 = 0u8;

#[derive(Debug, Clone)]
enum Module<'a> {
    FlipFlop(u8),
    Conjunction(HashMap<&'a str, u8>),
    Noop,
}
use Module::*;

fn parse_broadcaster(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(tag("broadcaster -> "), separated_list1(tag(", "), alpha1))(input)
}

fn parse_module(input: &str) -> IResult<&str, (&str, Module, Vec<&str>)> {
    let (input, module_type) = alt((
        complete::char('%').map(|_| FlipFlop(LOW)),
        complete::char('&').map(|_| Conjunction(HashMap::new())),
    ))(input)?;
    let (input, name) = alpha1(input)?;
    let (input, outputs) = preceded(tag(" -> "), separated_list1(tag(", "), alpha1))(input)?;
    Ok((input, (name, module_type, outputs)))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<(&str, Module, Vec<&str>)>)> {
    let (input, broadcast_outputs) = parse_broadcaster(input)?;
    let (input, _) = newline(input)?;
    let (input, modules) = separated_list1(newline, parse_module)(input)?;
    Ok((input, (broadcast_outputs, modules)))
}

pub fn process_part1(input: &str) -> String {
    let (input, (broadcast_outputs, modules)) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");

    let conjunction_module_names: HashSet<&str> = modules
        .iter()
        .filter_map(|(n, m, _)| match *m {
            Conjunction(_) => Some(*n),
            _ => None,
        })
        .collect();

    let mut conjunction_module_inputs: HashMap<&str, Vec<&str>> = conjunction_module_names
        .iter()
        .map(|n| (*n, vec![]))
        .collect();

    let mut module_map: HashMap<&str, (Module, Vec<&str>)> = HashMap::new();
    module_map.insert("broadcaster", (Noop, broadcast_outputs));

    for (name, module, outputs) in modules.into_iter() {
        for n in outputs
            .iter()
            .filter(|n| conjunction_module_names.contains(*n))
        {
            conjunction_module_inputs.get_mut(n).unwrap().push(name);
        }
        module_map.insert(name, (module, outputs));
    }

    // turn into input -> last signal in the final map
    for (name, inputs) in conjunction_module_inputs.into_iter() {
        let (ref mut m, _) = module_map.get_mut(&name).unwrap();
        if let Conjunction(mem) = m {
            *mem = inputs.into_iter().map(|i| (i, LOW)).collect()
        }
    }

    let mut low_signal_count: u64 = 0;
    let mut high_signal_count: u64 = 0;

    let initial_state = ("broadcaster", LOW, "button");

    for _ in 0..1000 {
        aoc_utils::complete_bfs(
            vec![initial_state].into_iter(),
            |(destination, signal, origin)| {
                debug_assert!(signal == LOW || signal == HIGH);
                if signal == LOW {
                    low_signal_count += 1;
                } else {
                    high_signal_count += 1;
                }

                return match module_map.get_mut(&destination) {
                    Some((FlipFlop(state), outputs)) if signal == LOW => {
                        *state = if *state == LOW { HIGH } else { LOW };
                        Some(
                            outputs
                                .iter()
                                .map(|o| (*o, *state, destination))
                                .collect_vec(),
                        )
                    }
                    Some((Conjunction(state), outputs)) => {
                        *state.entry(origin).or_default() = signal;
                        let out = !state.iter().all(|(_, v)| *v == HIGH) as u8;
                        Some(outputs.iter().map(|o| (*o, out, destination)).collect_vec())
                    }
                    Some((Noop, outputs)) => Some(
                        outputs
                            .iter()
                            .map(|o| (*o, signal, destination))
                            .collect_vec(),
                    ),
                    _ => None,
                };
            },
        );
    }
    (low_signal_count * high_signal_count).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, (broadcast_outputs, modules)) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");

    let conjunction_module_names: HashSet<&str> = modules
        .iter()
        .filter_map(|(n, m, _)| match *m {
            Conjunction(_) => Some(*n),
            _ => None,
        })
        .collect();

    let mut conjunction_module_inputs: HashMap<&str, Vec<&str>> = conjunction_module_names
        .iter()
        .map(|n| (*n, vec![]))
        .collect();

    let mut module_map: HashMap<&str, (Module, Vec<&str>)> = HashMap::new();
    module_map.insert("broadcaster", (Noop, broadcast_outputs));

    for (name, module, outputs) in modules.into_iter() {
        for n in outputs
            .iter()
            .filter(|n| conjunction_module_names.contains(*n))
        {
            conjunction_module_inputs.get_mut(n).unwrap().push(name);
        }
        module_map.insert(name, (module, outputs));
    }

    // turn into input -> last signal in the final map
    for (name, inputs) in conjunction_module_inputs.into_iter() {
        let (ref mut m, _) = module_map.get_mut(&name).unwrap();
        if let Conjunction(mem) = m {
            *mem = inputs.into_iter().map(|i| (i, LOW)).collect()
        }
    }

    let initial_state = ("broadcaster", LOW, "button");

    let mut seen: HashMap<&str, u8> = HashMap::new();
    let mut cycle_lengths: HashMap<&str, usize> = HashMap::new();
    let (feeder, (_, _)) = module_map
        .iter()
        .find(|(_, (_, o))| o.contains(&"rx"))
        .unwrap();
    let feeder = *feeder;

    for presses in 1.. {
        aoc_utils::complete_bfs(
            vec![initial_state].into_iter(),
            |(destination, signal, origin)| {
                debug_assert!(signal == LOW || signal == HIGH);

                if destination == feeder && signal == HIGH {
                    *seen.entry(origin).or_insert(0) += 1;
                    cycle_lengths.entry(origin).or_insert(presses);
                }

                return match module_map.get_mut(&destination) {
                    Some((FlipFlop(state), outputs)) if signal == LOW => {
                        *state = if *state == LOW { HIGH } else { LOW };
                        Some(
                            outputs
                                .iter()
                                .map(|o| (*o, *state, destination))
                                .collect_vec(),
                        )
                    }
                    Some((Conjunction(state), outputs)) => {
                        *state.entry(origin).or_default() = signal;
                        let out = !state.iter().all(|(_, v)| *v == HIGH) as u8;
                        Some(outputs.iter().map(|o| (*o, out, destination)).collect_vec())
                    }
                    Some((Noop, outputs)) => Some(
                        outputs
                            .iter()
                            .map(|o| (*o, signal, destination))
                            .collect_vec(),
                    ),
                    _ => None,
                };
            },
        );
        if !seen.is_empty() && seen.values().all(|v| *v > 10) {
            break;
        }
    }

    aoc_utils::lcm(cycle_lengths.into_values().collect_vec().as_slice()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        "32000000"
    )]
    #[case(
        "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        "11687500"
    )]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part1(input).as_str(), expected);
    }

    #[rstest]
    #[case("", "")]
    #[trace]
    #[ignore]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part2(input).as_str(), expected);
    }
}
