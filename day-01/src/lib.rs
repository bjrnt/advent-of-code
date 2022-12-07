pub fn process_part1(input: String) -> u32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

pub fn process_part2(input: String) -> u32 {
    let mut sums = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    sums.sort_by(|a, b| b.cmp(a));
    return sums.iter().take(3).sum::<u32>();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT.to_string()), 24000);
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT.to_string()), 45000);
    }
}
