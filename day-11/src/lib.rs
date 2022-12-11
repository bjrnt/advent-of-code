use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, digit1, newline, space1},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
enum Operation {
    Plus(Operand, Operand),
    Mul(Operand, Operand),
}

impl Operation {
    fn apply(&self, current_old: u64) -> u64 {
        match self {
            Operation::Plus(op1, op2) => op1.value(current_old) + op2.value(current_old),
            Operation::Mul(op1, op2) => op1.value(current_old) * op2.value(current_old),
        }
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Number(u64),
}

impl Operand {
    fn value(&self, current_old: u64) -> u64 {
        match self {
            Operand::Old => current_old,
            Operand::Number(v) => *v,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    index: usize,
    items: Vec<u64>,
    operation: Operation,
    test_divisible_by: u64,
    if_divisible: usize,
    if_not_divisible: usize,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {}: {:?}", self.index, self.items)
    }
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, operand1) = alt((digit1, tag("old")))(input)?;
    let (input, operation) = preceded(space1, alt((tag("* "), tag("+ "))))(input)?;
    let (input, operand2) = alt((digit1, tag("old")))(input)?;
    let op1 = match operand1 {
        "old" => Operand::Old,
        s => Operand::Number(s.parse::<u64>().unwrap()),
    };
    let op2 = match operand2 {
        "old" => Operand::Old,
        s => Operand::Number(s.parse::<u64>().unwrap()),
    };
    let op = match operation {
        "* " => Operation::Mul(op1, op2),
        "+ " => Operation::Plus(op1, op2),
        _ => panic!("unrecognized operation"),
    };
    Ok((input, op))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, idx) = preceded(tag("Monkey "), complete::u32)(input)?;
    let (input, _) = preceded(tag(":"), newline)(input)?;
    let (input, starting_items) = preceded(
        tag("  Starting items: "),
        separated_list1(tag(", "), complete::u64),
    )(input)?;
    let (input, _) = newline(input)?;
    let (input, operation) = preceded(tag("  Operation: new = "), parse_operation)(input)?;
    let (input, _) = newline(input)?;
    let (input, test_divisible_by) = preceded(tag("  Test: divisible by "), complete::u64)(input)?;
    let (input, _) = newline(input)?;
    let (input, if_divisible) =
        preceded(tag("    If true: throw to monkey "), complete::u64)(input)?;
    let (input, _) = newline(input)?;
    let (input, if_not_divisible) =
        preceded(tag("    If false: throw to monkey "), complete::u64)(input)?;

    Ok((
        input,
        Monkey {
            index: idx as usize,
            items: starting_items,
            operation,
            test_divisible_by,
            if_divisible: if_divisible as usize,
            if_not_divisible: if_not_divisible as usize,
        },
    ))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(tag("\n\n"), parse_monkey)(input)?;
    Ok((input, monkeys))
}

pub fn process_part1(input: &str) -> String {
    let (_, mut monkeys) = parse_monkeys(input).unwrap();
    let mut items_inspected = vec![0; monkeys.len()];
    let mut outbound: Vec<Vec<u64>> = vec![];
    for _ in 0..monkeys.len() {
        outbound.push(vec![]);
    }
    for _ in 1..=20 {
        for monkey_idx in 0..monkeys.len() {
            {
                let monkey = &mut monkeys[monkey_idx];
                for item_idx in 0..monkey.items.len() {
                    let item = &monkey.items[item_idx];
                    let new_worry_level = monkey.operation.apply(item.clone()) / 3;
                    let is_divisible = new_worry_level % monkey.test_divisible_by == 0;
                    items_inspected[monkey_idx] += 1;
                    if is_divisible {
                        outbound[monkey.if_divisible].push(new_worry_level);
                    } else {
                        outbound[monkey.if_not_divisible].push(new_worry_level);
                    }
                }
                monkey.items.clear();
            }

            for (monkey_idx, mut inbound) in outbound.iter_mut().enumerate() {
                let items = &mut monkeys[monkey_idx].items;
                items.append(&mut inbound);
            }
        }
    }

    items_inspected.sort();
    (items_inspected[items_inspected.len() - 1] * items_inspected[items_inspected.len() - 2])
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut monkeys) = parse_monkeys(input).unwrap();
    let mut items_inspected: Vec<u128> = vec![0; monkeys.len()];
    let mut outbound: Vec<Vec<u64>> = vec![];
    for _ in 0..monkeys.len() {
        outbound.push(vec![]);
    }
    let magic_trick = monkeys
        .iter()
        .map(|monkey| monkey.test_divisible_by)
        .product::<u64>();
    dbg!(magic_trick);
    for round in 1..=10_000 {
        for monkey_idx in 0..monkeys.len() {
            {
                let monkey = &mut monkeys[monkey_idx];
                for item_idx in 0..monkey.items.len() {
                    let item = &monkey.items[item_idx];
                    let new_worry_level = monkey.operation.apply(item.clone()) % magic_trick;
                    let is_divisible = new_worry_level % monkey.test_divisible_by == 0;
                    items_inspected[monkey_idx] += 1;
                    if is_divisible {
                        outbound[monkey.if_divisible].push(new_worry_level);
                    } else {
                        outbound[monkey.if_not_divisible].push(new_worry_level);
                    }
                }
                monkey.items.clear();
            }

            for (monkey_idx, mut inbound) in outbound.iter_mut().enumerate() {
                let items = &mut monkeys[monkey_idx].items;
                items.append(&mut inbound);
            }
        }

        println!("Round {round}:");
        for monkey in monkeys.iter() {
            println!("{}", monkey);
        }
    }

    items_inspected.sort();
    (items_inspected[items_inspected.len() - 1] * items_inspected[items_inspected.len() - 2])
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
