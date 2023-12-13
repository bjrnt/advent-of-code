use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
use Record::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Record {
    Operational,
    Damaged,
    Unknown,
}

fn parse_row(input: &str) -> IResult<&str, (Vec<Record>, Vec<usize>)> {
    separated_pair(
        many1(alt((
            complete::char('#').map(|_| Damaged),
            complete::char('.').map(|_| Operational),
            complete::char('?').map(|_| Unknown),
        ))),
        space1,
        separated_list1(tag(","), complete::u64.map(|v| v as usize)),
    )(input)
}

fn parse_rows(input: &str) -> IResult<&str, Vec<(Vec<Record>, Vec<usize>)>> {
    separated_list1(newline, parse_row)(input)
}

fn calculate_arrangements<'a>(
    cache: &mut HashMap<(&'a [Record], &'a [usize]), usize>,
    records: &'a [Record],
    groups: &'a [usize],
) -> usize {
    if records.is_empty() {
        return groups.is_empty() as usize;
    }

    if groups.is_empty() {
        return !records.contains(&Damaged) as usize;
    }

    if let Some(value) = cache.get(&(records, groups)) {
        return *value;
    }

    let mut permutations = 0;

    let next_record = *records.first().unwrap();

    if next_record == Operational || next_record == Unknown {
        permutations += calculate_arrangements(cache, &records[1..], groups);
    }

    if (next_record == Damaged || next_record == Unknown)
        && groups[0] <= records.len()
        && !records[..groups[0]].contains(&Operational)
        && (records.len() == groups[0] || records[groups[0]] != Damaged)
    {
        permutations += calculate_arrangements(
            cache,
            if groups[0] + 1 > records.len() {
                &[]
            } else {
                &records[groups[0] + 1..]
            },
            &groups[1..],
        )
    }

    cache.insert((records, groups), permutations);

    permutations
}

fn expand_row(records: Vec<Record>, groups: Vec<usize>) -> (Vec<Record>, Vec<usize>) {
    let new_records = itertools::Itertools::intersperse(
        std::iter::repeat(records.into_iter()).take(5),
        vec![Unknown].into_iter(),
    )
    .flatten()
    .collect_vec();
    let new_groups = std::iter::repeat(groups.into_iter())
        .take(5)
        .flatten()
        .collect_vec();
    (new_records, new_groups)
}

pub fn process_part1(input: &str) -> String {
    let (input, rows) = parse_rows(input).unwrap();
    debug_assert_eq!(input, "");
    rows.into_iter()
        .map(|(records, groups)| {
            calculate_arrangements(&mut HashMap::new(), records.as_slice(), groups.as_slice())
        })
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, rows) = parse_rows(input).unwrap();
    debug_assert_eq!(input, "");
    rows.into_iter()
        .map(|(records, groups)| expand_row(records, groups))
        .map(|(records, groups)| {
            calculate_arrangements(&mut HashMap::new(), records.as_slice(), groups.as_slice())
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        "21"
    )]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part1(input).as_str(), expected);
    }

    #[rstest]
    #[case("###.###.###.###.###.? 3,3,3,3,3,1", 1)]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    #[trace]
    fn test_arrangements(#[case] input: &str, #[case] expected: usize) {
        let mut cache = HashMap::new();
        let (_, (records, groups)) = parse_row(input).unwrap();
        assert_eq!(
            calculate_arrangements(&mut cache, records.as_slice(), groups.as_slice()),
            expected
        );
    }

    #[rstest]
    #[case("???.### 1,1,3", "1")]
    #[case(".??..??...?##. 1,1,3", "16384")]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", "1")]
    #[case("????.#...#... 4,1,1", "16")]
    #[case("????.######..#####. 1,6,5", "2500")]
    #[case("?###???????? 3,2,1", "506250")]
    #[trace]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part2(input).as_str(), expected);
    }
}
