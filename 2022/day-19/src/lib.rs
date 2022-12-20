use std::collections::VecDeque;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_robot_cost_ore: i32,
    clay_robot_cost_ore: i32,
    obsidian_robot_cost_ore: i32,
    obsidian_robot_cost_clay: i32,
    geode_robot_cost_ore: i32,
    geode_robot_cost_obsidian: i32,
}

#[derive(Debug, Clone)]
struct State {
    time_left: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    robot_ore: i32,
    robot_clay: i32,
    robot_obsidian: i32,
    robot_geode: i32,
}

impl State {
    fn add_resources(&mut self, time: i32) {
        self.ore += self.robot_ore * time;
        self.clay += self.robot_clay * time;
        self.obsidian += self.robot_obsidian * time;
        self.geode += self.robot_geode * time;
    }
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = preceded(tag("Blueprint "), complete::i32)(input)?;
    let (input, ore_robot_cost_ore) =
        preceded(tag(": Each ore robot costs "), complete::i32)(input)?;
    let (input, clay_robot_cost_ore) =
        preceded(tag(" ore. Each clay robot costs "), complete::i32)(input)?;
    let (input, obsidian_robot_cost_ore) =
        preceded(tag(" ore. Each obsidian robot costs "), complete::i32)(input)?;
    let (input, obsidian_robot_cost_clay) = preceded(tag(" ore and "), complete::i32)(input)?;
    let (input, geode_robot_cost_ore) =
        preceded(tag(" clay. Each geode robot costs "), complete::i32)(input)?;
    let (input, geode_robot_cost_obsidian) = preceded(tag(" ore and "), complete::i32)(input)?;
    let (input, _) = tag(" obsidian.")(input)?;

    Ok((
        input,
        Blueprint {
            id,
            ore_robot_cost_ore,
            clay_robot_cost_ore,
            obsidian_robot_cost_ore,
            obsidian_robot_cost_clay,
            geode_robot_cost_ore,
            geode_robot_cost_obsidian,
        },
    ))
}

fn parse_blueprints(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(newline, parse_blueprint)(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, blueprints) = parse_blueprints(input).unwrap();

    blueprints
        .par_iter()
        .map(|blueprint| {
            let total_time = 24;
            let initial_state = State {
                time_left: total_time,
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
                robot_ore: 1,
                robot_clay: 0,
                robot_obsidian: 0,
                robot_geode: 0,
            };

            let mut stack = VecDeque::from([initial_state]);
            let mut best_geodes = 0;

            while let Some(state) = stack.pop_back() {
                if state.time_left == 0 {
                    best_geodes = best_geodes.max(state.geode);
                    continue;
                }

                // wait
                if state.robot_geode > 0 {
                    let mut next_state = state.clone();
                    next_state.add_resources(state.time_left);
                    next_state.time_left = 0;
                    stack.push_back(next_state);
                }

                // build ore bot
                let time_required = if state.ore >= blueprint.ore_robot_cost_ore {
                    1
                } else {
                    1 + ((blueprint.ore_robot_cost_ore - state.ore) as f32 / state.robot_ore as f32)
                        .ceil() as i32
                };
                if time_required < state.time_left {
                    let mut next_state = state.clone();
                    next_state.add_resources(time_required);
                    next_state.time_left -= time_required;
                    next_state.ore -= blueprint.ore_robot_cost_ore;
                    next_state.robot_ore += 1;
                    stack.push_back(next_state);
                }

                // build clay robot
                let time_required = if state.ore >= blueprint.clay_robot_cost_ore {
                    1
                } else {
                    1 + ((blueprint.clay_robot_cost_ore - state.ore) as f32
                        / state.robot_ore as f32)
                        .ceil() as i32
                };
                if time_required < state.time_left {
                    let mut next_state = state.clone();
                    next_state.add_resources(time_required);
                    next_state.time_left -= time_required;
                    next_state.ore -= blueprint.clay_robot_cost_ore;
                    next_state.robot_clay += 1;
                    stack.push_back(next_state);
                }

                // build obsidian robot
                if state.robot_clay > 0 {
                    let time_required = if state.ore >= blueprint.obsidian_robot_cost_ore
                        && state.clay >= blueprint.obsidian_robot_cost_clay
                    {
                        1
                    } else {
                        1 + ((blueprint.obsidian_robot_cost_ore - state.ore) as f32
                            / state.robot_ore as f32)
                            .ceil()
                            .max(
                                ((blueprint.obsidian_robot_cost_clay - state.clay) as f32
                                    / state.robot_clay as f32)
                                    .ceil(),
                            ) as i32
                    };

                    if time_required < state.time_left {
                        let mut next_state = state.clone();
                        next_state.add_resources(time_required);
                        next_state.time_left -= time_required;
                        next_state.clay -= blueprint.obsidian_robot_cost_clay;
                        next_state.ore -= blueprint.obsidian_robot_cost_ore;
                        next_state.robot_obsidian += 1;
                        stack.push_back(next_state);
                    }
                }

                // build geode robot
                if state.robot_obsidian > 0 {
                    let time_required = if state.ore >= blueprint.geode_robot_cost_ore
                        && state.obsidian >= blueprint.geode_robot_cost_obsidian
                    {
                        1
                    } else {
                        1 + ((blueprint.geode_robot_cost_obsidian - state.obsidian) as f32
                            / state.robot_obsidian as f32)
                            .ceil()
                            .max(
                                ((blueprint.geode_robot_cost_ore - state.ore) as f32
                                    / state.robot_ore as f32)
                                    .ceil(),
                            ) as i32
                    };
                    if time_required < state.time_left {
                        let mut next_state = state.clone();
                        next_state.add_resources(time_required);
                        next_state.time_left -= time_required;
                        next_state.ore -= blueprint.geode_robot_cost_ore;
                        next_state.obsidian -= blueprint.geode_robot_cost_obsidian;
                        next_state.robot_geode += 1;
                        stack.push_back(next_state);
                    }
                }
            }

            blueprint.id * best_geodes
        })
        .sum::<i32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut blueprints) = parse_blueprints(input).unwrap();
    blueprints = blueprints.into_iter().take(3).collect();

    blueprints
        .par_iter()
        .map(|blueprint| {
            let total_time = 32;
            let initial_state = State {
                time_left: total_time,
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
                robot_ore: 1,
                robot_clay: 0,
                robot_obsidian: 0,
                robot_geode: 0,
            };

            // there is never a need to gain more of a resource per turn than what can be spent, this greatly reduces the branching factor closer to the end of the simulation
            let max_ore_cost = blueprint
                .ore_robot_cost_ore
                .max(blueprint.clay_robot_cost_ore)
                .max(blueprint.geode_robot_cost_ore)
                .max(blueprint.obsidian_robot_cost_ore);
            let max_clay_cost = blueprint.obsidian_robot_cost_clay;
            let max_obsidian_cost = blueprint.geode_robot_cost_obsidian;

            let mut stack = VecDeque::from([initial_state]);
            let mut best_geodes = 0;

            while let Some(state) = stack.pop_back() {
                if state.time_left == 0 {
                    best_geodes = best_geodes.max(state.geode as i64);
                    continue;
                }

                // wait
                if state.robot_geode > 0 {
                    let mut next_state = state.clone();
                    next_state.add_resources(state.time_left);
                    next_state.time_left = 0;
                    stack.push_back(next_state);
                }

                // build ore bot
                if max_ore_cost > state.robot_ore {
                    let time_required = if state.ore >= blueprint.ore_robot_cost_ore {
                        1
                    } else {
                        1 + ((blueprint.ore_robot_cost_ore - state.ore) as f32
                            / state.robot_ore as f32)
                            .ceil() as i32
                    };
                    if time_required < state.time_left {
                        let mut next_state = state.clone();
                        next_state.add_resources(time_required);
                        next_state.time_left -= time_required;
                        next_state.ore -= blueprint.ore_robot_cost_ore;
                        next_state.robot_ore += 1;
                        stack.push_back(next_state);
                    }
                }

                // build clay robot
                if max_clay_cost > state.robot_clay {
                    let time_required = if state.ore >= blueprint.clay_robot_cost_ore {
                        1
                    } else {
                        1 + ((blueprint.clay_robot_cost_ore - state.ore) as f32
                            / state.robot_ore as f32)
                            .ceil() as i32
                    };
                    if time_required < state.time_left {
                        let mut next_state = state.clone();
                        next_state.add_resources(time_required);
                        next_state.time_left -= time_required;
                        next_state.ore -= blueprint.clay_robot_cost_ore;
                        next_state.robot_clay += 1;
                        stack.push_back(next_state);
                    }
                }

                // build obsidian robot
                if max_obsidian_cost > state.robot_obsidian {
                    if state.robot_clay > 0 {
                        let time_required = if state.ore >= blueprint.obsidian_robot_cost_ore
                            && state.clay >= blueprint.obsidian_robot_cost_clay
                        {
                            1
                        } else {
                            1 + ((blueprint.obsidian_robot_cost_ore - state.ore) as f32
                                / state.robot_ore as f32)
                                .ceil()
                                .max(
                                    ((blueprint.obsidian_robot_cost_clay - state.clay) as f32
                                        / state.robot_clay as f32)
                                        .ceil(),
                                ) as i32
                        };

                        if time_required < state.time_left {
                            let mut next_state = state.clone();
                            next_state.add_resources(time_required);
                            next_state.time_left -= time_required;
                            next_state.clay -= blueprint.obsidian_robot_cost_clay;
                            next_state.ore -= blueprint.obsidian_robot_cost_ore;
                            next_state.robot_obsidian += 1;
                            stack.push_back(next_state);
                        }
                    }
                }

                // build geode robot
                if state.robot_obsidian > 0 {
                    let time_required = if state.ore >= blueprint.geode_robot_cost_ore
                        && state.obsidian >= blueprint.geode_robot_cost_obsidian
                    {
                        1
                    } else {
                        1 + ((blueprint.geode_robot_cost_obsidian - state.obsidian) as f32
                            / state.robot_obsidian as f32)
                            .ceil()
                            .max(
                                ((blueprint.geode_robot_cost_ore - state.ore) as f32
                                    / state.robot_ore as f32)
                                    .ceil(),
                            ) as i32
                    };
                    if time_required < state.time_left {
                        let mut next_state = state.clone();
                        next_state.add_resources(time_required);
                        next_state.time_left -= time_required;
                        next_state.ore -= blueprint.geode_robot_cost_ore;
                        next_state.obsidian -= blueprint.geode_robot_cost_obsidian;
                        next_state.robot_geode += 1;
                        stack.push_back(next_state);
                    }
                }
            }

            best_geodes
        })
        .product::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "33");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "3472");
    }
}
