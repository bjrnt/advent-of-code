use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

fn parse_card(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = nom::character::complete::u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    separated_pair(
        separated_list1(space1, nom::character::complete::u32),
        terminated(preceded(space1, tag("|")), space1),
        separated_list1(space1, nom::character::complete::u32),
    )(input)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<(Vec<u32>, Vec<u32>)>> {
    separated_list1(newline, parse_card)(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, cards) = parse_cards(input).unwrap();
    cards
        .into_iter()
        .map(|(winning, my)| {
            let winning_set: HashSet<u32> = winning.into_iter().collect();
            let num_winning_cards =
                my.into_iter().filter(|c| winning_set.contains(c)).count() as u32;
            if num_winning_cards == 0 {
                return 0;
            };
            (2u32).pow(num_winning_cards - 1)
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, cards) = parse_cards(input).unwrap();
    let cards: HashMap<u32, (Vec<u32>, Vec<u32>)> = cards
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i as u32 + 1, v))
        .collect();
    let mut match_cache = HashMap::new();

    let mut to_process: VecDeque<u32> = (1..=cards.len() as u32).collect();

    let mut total_cards = 0;
    while let Some(card_number) = to_process.pop_front() {
        total_cards += 1;

        let num_winning_cards = if let Entry::Vacant(e) = match_cache.entry(card_number) {
            let (winning, my) = cards.get(&card_number).unwrap();
            let winning_set: HashSet<u32> = winning.clone().into_iter().collect();
            let num_winning_cards =
                my.iter().filter(|c| winning_set.contains(c)).count() as u32;
            e.insert(num_winning_cards);
            num_winning_cards
        } else {
            *match_cache.get(&card_number).unwrap()
        };

        for extra_card in card_number + 1..=card_number + num_winning_cards {
            to_process.push_back(extra_card);
        }
    }
    total_cards.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "13");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "30");
    }
}
