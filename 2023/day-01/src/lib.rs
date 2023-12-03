pub fn process_part1(input: &str) -> String {
    format!(
        "{}",
        input.lines().into_iter().map(process_line).sum::<u32>()
    )
}

fn process_line(line: &str) -> u32 {
    let digits = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();
    digits[0] * 10 + digits.last().unwrap()
}

pub fn process_part2(input: &str) -> String {
    format!(
        "{}",
        input
            .lines()
            .into_iter()
            .map(process_line_part2)
            .sum::<u32>()
    )
}

fn process_line_part2(line: &str) -> u32 {
    let digits = line
        .char_indices()
        .filter_map(|(i, c)| {
            let numerical_digit = c.to_digit(10);
            if numerical_digit.is_some() {
                return numerical_digit;
            };
            return parse_digit_letters(&line[i..line.len()]);
        })
        .collect::<Vec<u32>>();
    digits[0] * 10 + digits.last().unwrap()
}

fn parse_digit_letters(input: &str) -> Option<u32> {
    if input.starts_with("one") {
        Some(1)
    } else if input.starts_with("two") {
        Some(2)
    } else if input.starts_with("three") {
        Some(3)
    } else if input.starts_with("four") {
        Some(4)
    } else if input.starts_with("five") {
        Some(5)
    } else if input.starts_with("six") {
        Some(6)
    } else if input.starts_with("seven") {
        Some(7)
    } else if input.starts_with("eight") {
        Some(8)
    } else if input.starts_with("nine") {
        Some(9)
    } else {
        None
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
