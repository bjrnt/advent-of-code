use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::{separated_list0, separated_list1},
    sequence::preceded,
    IResult,
};

fn parse_valve(input: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
    let (input, valve) = preceded(tag("Valve "), alpha1)(input)?;
    let (input, flow_rate) =
        preceded(tag(" has flow rate="), nom::character::complete::u32)(input)?;

    let (input, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(input)?;
    let (input, neighbors) = separated_list0(tag(", "), alpha1)(input)?;
    Ok((input, (valve, flow_rate, neighbors)))
}

fn parse_valves(input: &str) -> IResult<&str, Vec<(&str, u32, Vec<&str>)>> {
    separated_list1(newline, parse_valve)(input)
}

fn dfs_helper<'a>(
    time: u32,
    current_location: &'a str,
    flow_rate: u32,
    total_flowed: u32,
    open_valves: &HashSet<&str>,
    graph: &'a HashMap<&str, Vec<&str>>,
    flows: &HashMap<&str, u32>,
    cache: &mut HashMap<(u32, &'a str, u32), u32>,
) -> Option<u32> {
    if time == 30 {
        return Some(total_flowed);
    }

    let cache_key = (time, current_location, flow_rate);
    if let Some(cached_value) = cache.get(&cache_key) {
        if *cached_value >= total_flowed {
            return None;
        }
    }
    cache.insert(cache_key, total_flowed);

    let current_valve = graph.get(current_location).unwrap();

    let best_result_open_current =
        if *flows.get(current_location).unwrap() > 0 && !open_valves.contains(current_location) {
            let mut new_open_valves = open_valves.clone();
            new_open_valves.insert(current_location);

            let new_total_flowed = total_flowed + flow_rate;
            let new_flow_rate = flow_rate + flows.get(current_location).unwrap();
            dfs_helper(
                time + 1,
                current_location,
                new_flow_rate,
                new_total_flowed,
                &new_open_valves,
                graph,
                flows,
                cache,
            )
        } else {
            None
        };

    let best_result_to_neighbor = current_valve
        .iter()
        .filter_map(|neighbor| {
            dfs_helper(
                time + 1,
                neighbor,
                flow_rate,
                total_flowed + flow_rate,
                open_valves,
                graph,
                flows,
                cache,
            )
        })
        .max();

    best_result_open_current.max(best_result_to_neighbor)
}

fn dfs_helper2<'a>(
    time: u32,
    my_location: &'a str,
    elephant_location: &'a str,
    flow_rate: u32,
    total_flowed: u32,
    open_valves: &HashSet<&str>,
    graph: &'a HashMap<&str, Vec<&str>>,
    flows: &HashMap<&str, u32>,
    cache: &mut HashMap<(u32, &'a str, &'a str, u32), u32>,
) -> Option<u32> {
    if time == 26 {
        return Some(total_flowed);
    }

    let cache_key = (time, my_location, elephant_location, flow_rate);
    if let Some(cached_value) = cache.get(&cache_key) {
        if *cached_value >= total_flowed {
            return None;
        }
    }
    cache.insert(cache_key, total_flowed);

    let my_current_valve = graph.get(my_location).unwrap();
    let my_flow_rate = flows.get(my_location).unwrap();
    let elephant_current_valve = graph.get(elephant_location).unwrap();
    let elephant_flow_rate = flows.get(elephant_location).unwrap();

    let can_open_my_valve = *my_flow_rate > 0 && !open_valves.contains(my_location);
    let can_open_elephant_valve =
        *elephant_flow_rate > 0 && !open_valves.contains(elephant_location);

    let mut results = Vec::new();

    // i open, elephant moves
    if can_open_my_valve {
        let mut new_open_valves = open_valves.clone();
        new_open_valves.insert(my_location);

        for new_elephant_location in elephant_current_valve.iter() {
            results.push(dfs_helper2(
                time + 1,
                my_location,
                *new_elephant_location,
                flow_rate + my_flow_rate,
                total_flowed + flow_rate,
                &new_open_valves,
                graph,
                flows,
                cache,
            ));
        }
    }

    // i move, elephant opens
    if can_open_elephant_valve {
        let mut new_open_valves = open_valves.clone();
        new_open_valves.insert(elephant_location);

        for my_new_location in my_current_valve.iter() {
            results.push(dfs_helper2(
                time + 1,
                *my_new_location,
                elephant_location,
                flow_rate + elephant_flow_rate,
                total_flowed + flow_rate,
                &new_open_valves,
                graph,
                flows,
                cache,
            ));
        }
    }

    // both open
    if can_open_elephant_valve && can_open_my_valve && my_location != elephant_location {
        let mut new_open_valves = open_valves.clone();
        new_open_valves.insert(elephant_location);
        new_open_valves.insert(my_location);

        results.push(dfs_helper2(
            time + 1,
            my_location,
            elephant_location,
            flow_rate + my_flow_rate + elephant_flow_rate,
            total_flowed + flow_rate,
            &new_open_valves,
            graph,
            flows,
            cache,
        ));
    }

    // both move
    for new_elephant_location in elephant_current_valve.iter() {
        for my_new_location in my_current_valve.iter() {
            results.push(dfs_helper2(
                time + 1,
                my_new_location,
                new_elephant_location,
                flow_rate,
                total_flowed + flow_rate,
                open_valves,
                graph,
                flows,
                cache,
            ));
        }
    }

    results.into_iter().flatten().max()
}

pub fn process_part1(input: &str) -> String {
    let (_, valves) = parse_valves(input).unwrap();

    let flows: HashMap<&str, u32> = HashMap::from_iter(valves.iter().map(|(v, f, _)| (*v, *f)));
    let graph: HashMap<&str, Vec<&str>> =
        HashMap::from_iter(valves.into_iter().map(|(v, _, n)| (v, n)));
    let mut cache: HashMap<(u32, &str, u32), u32> = HashMap::new();

    let result = dfs_helper(0, "AA", 0, 0, &HashSet::new(), &graph, &flows, &mut cache)
        .expect("couldn't find a solution at all");
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, valves) = parse_valves(input).unwrap();

    let flows: HashMap<&str, u32> = HashMap::from_iter(valves.iter().map(|(v, f, _)| (*v, *f)));
    let graph: HashMap<&str, Vec<&str>> =
        HashMap::from_iter(valves.into_iter().map(|(v, _, n)| (v, n)));
    let mut cache: HashMap<(u32, &str, &str, u32), u32> = HashMap::new();

    let result = dfs_helper2(
        0,
        "AA",
        "AA",
        0,
        0,
        &HashSet::new(),
        &graph,
        &flows,
        &mut cache,
    )
    .expect("coudn't find a solution at all");
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "1651");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "1707");
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
