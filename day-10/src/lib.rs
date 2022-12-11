use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Addx(_) => 2,
            Noop => 1,
        }
    }
}

use Instruction::*;

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(
        newline,
        alt((
            tag("noop").map(|_| Noop),
            preceded(tag("addx "), complete::i32).map(|v| Addx(v)),
        )),
    )(input)?;
    return Ok((input, instructions));
}

pub fn process_part1(input: &str) -> String {
    let measure_signal_strength_at: HashSet<i32> = HashSet::from([20, 60, 100, 140, 180, 220]);
    let (_, ins) = parse_instructions(input).unwrap();

    let mut instructions = ins.iter();
    let mut x: i32 = 1;
    let mut current_instruction = None;
    let mut cycles_remaining: u32 = 0;
    let mut signal_strengths: Vec<i32> = vec![];

    for cycle in 1.. {
        if current_instruction.is_none() {
            current_instruction = instructions.next();
            cycles_remaining = match current_instruction {
                Some(instr) => instr.cycles(),
                None => break,
            };
        }

        if measure_signal_strength_at.contains(&cycle) {
            signal_strengths.push(cycle * x);
        }

        cycles_remaining -= 1;

        if cycles_remaining == 0 {
            match current_instruction.unwrap() {
                Instruction::Addx(val) => x += val,
                Instruction::Noop => (),
            }
            current_instruction = None;
        }
    }

    signal_strengths.iter().sum::<i32>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, ins) = parse_instructions(input).unwrap();

    let mut crt: [char; 40 * 6] = ['.'; 40 * 6];

    let mut instructions = ins.iter();
    let mut x: i32 = 1;
    let mut current_instruction = None;
    let mut cycles_remaining: u32 = 0;

    for cycle in 1.. {
        if current_instruction.is_none() {
            current_instruction = instructions.next();
            cycles_remaining = match current_instruction {
                Some(instr) => instr.cycles(),
                None => break,
            };
        }

        let crt_pos: usize = cycle - 1;
        if ((crt_pos % 40) as i32 - x).abs() <= 1 {
            crt[crt_pos] = '#';
        }

        cycles_remaining -= 1;

        if cycles_remaining == 0 {
            match current_instruction.unwrap() {
                Addx(val) => x += val,
                Noop => (),
            }
            current_instruction = None;
        }
    }

    crt.chunks(40)
        .into_iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "13140");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    fn part2() {
        print!("{}", process_part2(EXAMPLE_INPUT));
        assert_eq!(
            process_part2(EXAMPLE_INPUT),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
