use itertools::Itertools;

fn to_priority(c: char) -> u32 {
    match c {
        c @ 'a'..='z' => c as u32 - 'a' as u32 + 1,
        c @ 'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("unacceptable character '{c}'"),
    }
}

pub fn process_part1(input: String) -> u32 {
    input
        .lines()
        .map(|rucksack| {
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            let common_char = left.chars().find(|c| right.contains(*c)).unwrap();
            to_priority(common_char)
        })
        .sum()
}

pub fn process_part2(input: String) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|elf_team| {
            let [a, b, c] = elf_team.collect::<Vec<&str>>()[..] else {
                panic!("unexpected elf team size")
            };
            let common_char = a
                .chars()
                .find(|ch| b.contains(*ch) && c.contains(*ch))
                .unwrap();
            to_priority(common_char)
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT.to_string()), 157);
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT.to_string()), 70);
    }
}
