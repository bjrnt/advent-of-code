use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};

#[derive(Debug)]
enum Value {
    Old,
    Num(u64),
}

impl Value {
    fn eval(&self, old: u64) -> u64 {
        match self {
            Value::Old => old,
            Value::Num(v) => *v,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Mul((Value, Value)),
    Add((Value, Value)),
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Mul((a, b)) => a.eval(old) * b.eval(old),
            Operation::Add((a, b)) => a.eval(old) + b.eval(old),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    inspect_count: u64,
}

impl Monkey {
    fn inspect(&mut self) -> u64 {
        self.inspect_count += 1;
        self.operation.apply(self.items.pop_front().unwrap())
    }

    fn test(&self, item: u64) -> u64 {
        self.test.apply(item)
    }
}

#[derive(Debug)]
struct Test {
    divisible: u64,
    true_recipient: u64,
    false_recipient: u64,
}

impl Test {
    fn apply(&self, item: u64) -> u64 {
        if item % self.divisible == 0 {
            self.true_recipient
        } else {
            self.false_recipient
        }
    }
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((
        tag("old").map(|_| Value::Old),
        complete::u64.map(Value::Num),
    ))(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, value1) = preceded(tag("Operation: new = "), value)(input)?;
    let (input, operator) = delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(input)?;
    let (input, value2) = value(input)?;

    let result = match operator {
        "*" => Operation::Mul((value1, value2)),
        "+" => Operation::Add((value1, value2)),
        _ => panic!("unknown operator"),
    };

    Ok((input, result))
}

fn test_parser(input: &str) -> IResult<&str, Test> {
    let (input, divisible) = preceded(tag("Test: divisible by "), complete::u64)(input)?;
    let (input, true_recipient) = preceded(
        multispace1,
        preceded(tag("If true: throw to monkey "), complete::u64),
    )(input)?;
    let (input, false_recipient) = preceded(
        multispace1,
        preceded(tag("If false: throw to monkey "), complete::u64),
    )(input)?;
    Ok((
        input,
        Test {
            divisible,
            true_recipient,
            false_recipient,
        },
    ))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), complete::u32, tag(":"))(input)?;
    let (input, items) = preceded(
        multispace1,
        preceded(
            tag("Starting items: "),
            separated_list1(tag(", "), complete::u64),
        ),
    )(input)?;
    let (input, op) = delimited(multispace1, operation, multispace1)(input)?;
    let (input, test) = test_parser(input)?;
    Ok((
        input,
        Monkey {
            items: VecDeque::from(items),
            operation: op,
            test,
            inspect_count: 0,
        },
    ))
}

fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n\n"), monkey)(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, mut monkeys) = monkeys(input).unwrap();

    for _round in 0..20 {
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey = monkeys.get_mut(monkey_index).unwrap();
                let item = monkey.inspect() / 3;
                let monkey_to_send_to = monkey.test(item);
                monkeys
                    .get_mut(monkey_to_send_to as usize)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.inspect_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.inspect_count)
        .product::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut monkeys) = monkeys(input).unwrap();
    let least_common_multiple = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .product::<u64>();

    for _round in 0..10_000 {
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey = monkeys.get_mut(monkey_index).unwrap();
                let item = monkey.inspect() % least_common_multiple;
                let monkey_to_send_to = monkey.test(item);
                monkeys
                    .get_mut(monkey_to_send_to as usize)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.inspect_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.inspect_count)
        .product::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "10605");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "2713310158");
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
