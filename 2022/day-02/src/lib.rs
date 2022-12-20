use std::{cmp::Ordering, str::FromStr, sync::RwLock};

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Greater)
        } else if self == &Move::Rock && other == &Move::Scissors {
            Some(Ordering::Less)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(format!("unrecognized move: {}", s)),
        }
    }
}

impl Move {
    pub fn points(&self) -> u32 {
        *self as u32
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Outcome {
    Draw = 3,
    Win = 6,
    Loss = 0,
}

impl Outcome {
    pub fn from_moves(me: &Move, opponent: &Move) -> Result<Self, String> {
        match me.partial_cmp(opponent) {
            Some(Ordering::Equal) => Ok(Outcome::Draw),
            Some(Ordering::Greater) => Ok(Outcome::Win),
            Some(Ordering::Less) => Ok(Outcome::Loss),
            None => Err("could not compare moves".to_string()),
        }
    }

    pub fn points(&self) -> u32 {
        *self as u32
    }
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(format!("unrecognized outcome: {}", s)),
        }
    }
}

fn calculate_my_move(opponent: &Move, desired_outcome: &Outcome) -> Move {
    match desired_outcome {
        Outcome::Draw => opponent.clone(),
        Outcome::Win => match opponent {
            Move::Scissors => Move::Rock,
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
        },
        Outcome::Loss => match opponent {
            Move::Scissors => Move::Paper,
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
        },
    }
}

pub fn process_part1(input: String) -> u32 {
    input
        .lines()
        .map(|game| {
            let (their_move, my_move) = game.split_once(" ").unwrap();
            let them = their_move.parse::<Move>().unwrap();
            let me = my_move.parse::<Move>().unwrap();
            me.points() + Outcome::from_moves(&me, &them).unwrap().points()
        })
        .sum::<u32>()
}

pub fn process_part2(input: String) -> u32 {
    input
        .lines()
        .map(|game| {
            let (their_move, desired_outcome) = game.split_once(" ").unwrap();
            let them = their_move.parse::<Move>().unwrap();
            let outcome = desired_outcome.parse::<Outcome>().unwrap();
            let me = calculate_my_move(&them, &outcome);
            outcome.points() + me.points()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT.to_string()), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT.to_string()), 12);
    }
}
