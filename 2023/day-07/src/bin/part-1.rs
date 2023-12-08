use itertools::Itertools;
use nom::{
    character::complete::{alphanumeric1, newline, space1},
    multi::separated_list1,
    IResult,
};
use std::collections::HashMap;
use std::fs;

fn to_hand_type(cards: &Vec<u8>) -> u8 {
    let mut freq: HashMap<u8, usize> = HashMap::new();

    cards.iter().for_each(|c| {
        let count = freq.entry(*c).or_insert(0);
        *count += 1;
    });

    let counts: Vec<_> = freq.values().sorted().collect();

    match counts.as_slice() {
        [5] => 7,          // five of a kind
        [1, 4] => 6,       // four of a kind
        [2, 3] => 5,       // full house
        [1, 1, 3] => 4,    // three of a kind
        [1, 2, 2] => 3,    // two pair
        [1, 1, 1, 2] => 2, // pair
        _ => 1,            // high card
    }
}

fn to_card(value: char) -> Option<u8> {
    match value {
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'T' => Some(10),
        'J' => Some(11),
        'Q' => Some(12),
        'K' => Some(13),
        'A' => Some(14),
        _ => None,
    }
}

fn parse_hand_and_bet(input: &str) -> IResult<&str, (Vec<u8>, u32)> {
    let (input, card_chars) = alphanumeric1(input)?;
    let cards = card_chars.chars().map(|c| to_card(c).unwrap()).collect();
    let (input, _) = space1(input)?;
    let (input, bet) = nom::character::complete::u32(input)?;
    Ok((input, (cards, bet)))
}

fn parse_hands_and_bets(input: &str) -> IResult<&str, Vec<(Vec<u8>, u32)>> {
    separated_list1(newline, parse_hand_and_bet)(input)
}

fn process_part1(input: &str) -> String {
    let (_, hands_and_bets) = parse_hands_and_bets(input).unwrap();
    hands_and_bets
        .into_iter()
        .map(|(cards, bet)| (to_hand_type(&cards), cards, bet))
        .sorted()
        .enumerate()
        .map(|(rank, (_, _, bet))| (rank as u32 + 1) * bet)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    const EXAMPLE_INPUTS: [(&str, u8); 5] = [
        ("32T3K 765", 2),
        ("T55J5 684", 4),
        ("KK677 28", 3),
        ("KTJJT 220", 3),
        ("QQQJA 483", 4),
    ];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "6440");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1) in EXAMPLE_INPUTS.iter() {
            let (_, (cards, _)) = parse_hand_and_bet(input).unwrap();
            assert_eq!(to_hand_type(&cards), *answer_part1);
        }
    }
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(file.as_str()));
}
