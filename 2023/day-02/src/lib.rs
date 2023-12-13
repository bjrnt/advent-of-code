use nom::{
    branch::alt, bytes::complete::tag, character::complete::newline, combinator::opt, multi::many1,
    multi::separated_list1, sequence::preceded, IResult, Parser,
};

#[derive(Debug)]
struct Game {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

#[derive(Debug)]
enum Color {
    Green,
    Red,
    Blue,
}

fn parse_color(input: &str) -> IResult<&str, (u32, Color)> {
    let (input, count) = nom::character::complete::u32(input)?;
    let (input, color) = alt((
        tag(" blue").map(|_| Color::Blue),
        tag(" green").map(|_| Color::Green),
        tag(" red").map(|_| Color::Red),
    ))(input)?;
    let (input, _) = opt(tag(", "))(input)?;
    Ok((input, (count, color)))
}

fn parse_round(input: &str) -> IResult<&str, Game> {
    let (input, colors) = many1(parse_color)(input)?;
    let mut game = Game {
        red: None,
        blue: None,
        green: None,
    };
    for (count, color) in colors {
        match color {
            Color::Red => game.red = Some(count),
            Color::Green => game.green = Some(count),
            Color::Blue => game.blue = Some(count),
        };
    }
    Ok((input, game))
}

fn parse_rounds(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(tag("; "), parse_round)(input)
}

fn parse_game(input: &str) -> IResult<&str, (u32, Vec<Game>)> {
    let (input, id) = preceded(tag("Game "), nom::character::complete::u32)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rounds) = parse_rounds(input)?;
    Ok((input, (id, rounds)))
}

fn parse_games(input: &str) -> IResult<&str, Vec<(u32, Vec<Game>)>> {
    separated_list1(newline, parse_game)(input)
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn process_part1(input: &str) -> String {
    let (_, games) = parse_games(input).unwrap();
    games
        .iter()
        .filter_map(|(id, rounds)| {
            if !rounds.iter().all(|round| {
                (round.blue.is_none() || round.blue.is_some_and(|v| v <= MAX_BLUE))
                    && (round.red.is_none() || round.red.is_some_and(|v| v <= MAX_RED))
                    && (round.green.is_none() || round.green.is_some_and(|v| v <= MAX_GREEN))
            }) {
                return None;
            }
            Some(id)
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, games) = parse_games(input).unwrap();
    games
        .iter()
        .map(|(_, rounds)| {
            let fewest_red = rounds.iter().max_by_key(|r| r.red).unwrap().red.unwrap();
            let fewest_green = rounds
                .iter()
                .max_by_key(|r| r.green)
                .unwrap()
                .green
                .unwrap();
            let fewest_blue = rounds.iter().max_by_key(|r| r.blue).unwrap().blue.unwrap();
            fewest_red * fewest_green * fewest_blue
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "8");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "2286");
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
