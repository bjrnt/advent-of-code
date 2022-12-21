use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::{
        complete::{self, alpha1, anychar, multispace1, newline},
        is_alphabetic,
    },
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
enum Monkey<'a> {
    Value(i64),
    Calculate(&'a str, Operation, &'a str),
}

fn parse_monkey(input: &str) -> IResult<&str, (&str, Monkey)> {
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    if is_alphabetic(input.chars().nth(0).unwrap() as u8) {
        let (input, monkey_a) = alpha1(input)?;
        let (input, operator) = delimited(tag(" "), anychar, tag(" "))(input)?;
        let (input, monkey_b) = alpha1(input)?;
        let op = match operator {
            '+' => Operation::Add,
            '-' => Operation::Sub,
            '/' => Operation::Div,
            '*' => Operation::Mul,
            _ => panic!("unknown operation"),
        };
        Ok((input, (name, Monkey::Calculate(monkey_a, op, monkey_b))))
    } else {
        let (input, val) = complete::i64(input)?;
        Ok((input, (name, Monkey::Value(val))))
    }
}

fn parse_monkeys(input: &str) -> IResult<&str, HashMap<&str, Monkey>> {
    let (input, monkeys) = separated_list1(multispace1, parse_monkey)(input)?;
    Ok((input, HashMap::from_iter(monkeys.into_iter())))
}

fn resolve_monkeys(monkeys: &HashMap<&str, Monkey>, name: &str) -> i64 {
    let monkey = monkeys.get(name).unwrap();
    match monkey {
        Monkey::Value(val) => *val,
        Monkey::Calculate(monkey_a, op, monkey_b) => {
            let val_a = resolve_monkeys(monkeys, &monkey_a);
            let val_b = resolve_monkeys(monkeys, &monkey_b);
            match op {
                Operation::Add => val_a + val_b,
                Operation::Sub => val_a - val_b,
                Operation::Mul => val_a * val_b,
                Operation::Div => val_a / val_b,
            }
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let (_, monkeys) = parse_monkeys(input).unwrap();
    resolve_monkeys(&monkeys, "root").to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut monkeys) = parse_monkeys(input).unwrap();
    let mut value = 150;
    loop {
        monkeys.insert("humn", Monkey::Value(value));
        let Monkey::Calculate(a, _, b)= monkeys.get("root").unwrap() else {
        panic!("oh no");
    };
        let monkey_a = resolve_monkeys(&monkeys, a);
        let monkey_b = resolve_monkeys(&monkeys, b);
        if monkey_a == monkey_b {
            break;
        } else {
            println!("{} != {}", monkey_a, monkey_b);
            value += 1.max((monkey_a - monkey_b) / 100000);
        }
    }
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "152");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "301");
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
