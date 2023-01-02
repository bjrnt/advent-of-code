use std::iter::Sum;
use std::str::FromStr;

use itertools::unfold;
use nom::character::complete::char;
use nom::Parser;
use nom::{branch::alt, multi::many1, IResult};

fn snafu_digits(input: &str) -> IResult<&str, Vec<i8>> {
    many1(alt((
        char('2').map(|_| 2),
        char('1').map(|_| 1),
        char('0').map(|_| 0),
        char('-').map(|_| -1),
        char('=').map(|_| -2),
    )))(input)
}

fn snafu(input: &str) -> IResult<&str, Snafu> {
    let (_, digits) = snafu_digits(input)?;
    let decimal = digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, item)| {
            let contribution = *item as i64 * 5i64.pow(index as u32);
            acc + contribution
        });
    Ok((input, Snafu { decimal }))
}

#[derive(Debug, PartialEq)]
struct Snafu {
    decimal: i64,
}

impl FromStr for Snafu {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, snafu) = snafu(s).map_err(|err| {
            dbg!(err);
            "unparsable"
        })?;
        Ok(snafu)
    }
}

impl Snafu {
    fn to_snafu_string(&self) -> String {
        let result = unfold(self.decimal, |num| {
            if num == &0 {
                None
            } else {
                match *num % 5 {
                    0 => {
                        *num /= 5;
                        Some('0')
                    }
                    1 => {
                        *num -= 1;
                        *num /= 5;
                        Some('1')
                    }
                    2 => {
                        *num -= 2;
                        *num /= 5;
                        Some('2')
                    }
                    3 => {
                        *num -= -2;
                        *num /= 5;
                        Some('=')
                    }
                    4 => {
                        *num -= -1;
                        *num /= 5;
                        Some('-')
                    }
                    _ => panic!("impossible"),
                }
            }
        })
        .collect::<String>();
        result.chars().rev().collect()
    }
}

impl Sum<Snafu> for Snafu {
    fn sum<I: Iterator<Item = Snafu>>(iter: I) -> Self {
        Snafu {
            decimal: iter.map(|s| s.decimal).sum(),
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let sum = input
        .split("\n")
        .map(|s| s.parse::<Snafu>().unwrap())
        .sum::<Snafu>();
    sum.to_snafu_string()
}

pub fn process_part2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn parser_works() {
        assert_eq!("1=-0-2".parse::<Snafu>().unwrap(), Snafu { decimal: 1747 });
        assert_eq!("12111".parse::<Snafu>().unwrap(), Snafu { decimal: 906 });
        assert_eq!("2=0=".parse::<Snafu>().unwrap(), Snafu { decimal: 198 });
        assert_eq!("21".parse::<Snafu>().unwrap(), Snafu { decimal: 11 });
        assert_eq!("2=01".parse::<Snafu>().unwrap(), Snafu { decimal: 201 });
        assert_eq!("111".parse::<Snafu>().unwrap(), Snafu { decimal: 31 });
        assert_eq!("20012".parse::<Snafu>().unwrap(), Snafu { decimal: 1257 });
        assert_eq!("112".parse::<Snafu>().unwrap(), Snafu { decimal: 32 });
        assert_eq!("1=-1=".parse::<Snafu>().unwrap(), Snafu { decimal: 353 });
        assert_eq!("1-12".parse::<Snafu>().unwrap(), Snafu { decimal: 107 });
        assert_eq!("12".parse::<Snafu>().unwrap(), Snafu { decimal: 7 });
        assert_eq!("1=".parse::<Snafu>().unwrap(), Snafu { decimal: 3 });
        assert_eq!("122".parse::<Snafu>().unwrap(), Snafu { decimal: 37 });
    }

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "4890");
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "");
    }
}
