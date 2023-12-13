pub fn process_part1(input: &str) -> String {
    input.lines().map(process_line).sum::<u32>().to_string()
}

fn process_line(line: &str) -> u32 {
    let digits = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();
    digits[0] * 10 + digits.last().unwrap()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(process_line_part2)
        .sum::<u32>()
        .to_string()
}

fn process_line_part2(line: &str) -> u32 {
    let digits = line
        .char_indices()
        .filter_map(|(i, c)| c.to_digit(10).or_else(|| parse_digit_letters(&line[i..])))
        .collect::<Vec<u32>>();
    digits[0] * 10 + digits.last().unwrap()
}

fn parse_digit_letters(input: &str) -> Option<u32> {
    match input {
        i if i.starts_with("one") => Some(1),
        i if i.starts_with("two") => Some(2),
        i if i.starts_with("three") => Some(3),
        i if i.starts_with("four") => Some(4),
        i if i.starts_with("five") => Some(5),
        i if i.starts_with("six") => Some(6),
        i if i.starts_with("seven") => Some(7),
        i if i.starts_with("eight") => Some(8),
        i if i.starts_with("nine") => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "142");
    }

    #[test]
    fn part2() {
        assert_eq!(
            process_part2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            "281"
        );
    }
}
