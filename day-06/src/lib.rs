use std::collections::{HashSet, LinkedList};

pub fn process_part1(input: &str) -> u32 {
    let mut chars_in_window = LinkedList::new();
    let mut window_set = HashSet::new();
    let mut start_marker = None;
    for (i, c) in input.chars().enumerate() {
        while window_set.contains(&c) {
            window_set.remove(&chars_in_window.pop_front().unwrap());
        }
        if chars_in_window.len() == 4 {
            window_set.remove(&chars_in_window.pop_front().unwrap());
        }
        chars_in_window.push_back(c);
        window_set.insert(c);
        if window_set.len() == 4 {
            start_marker = Some(i as u32 + 1);
            break;
        }
    }
    start_marker.expect("could not find start marker")
}

pub fn process_part2(input: &str) -> u32 {
    let window_size = 14;
    let chars = input.chars().collect::<Vec<char>>();
    let sequence = chars
        .windows(window_size)
        .enumerate()
        .find(|(_i, window)| {
            let set = window.iter().collect::<HashSet<&char>>();
            window.len() == set.len()
        })
        .expect("could not find start marker");
    sequence.0 as u32 + window_size as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUTS_P1: [(&'static str, u32); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    const EXAMPLE_INPUTS_P2: [(&'static str, u32); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];

    #[test]
    fn part1() {
        for (input, idx) in EXAMPLE_INPUTS_P1.iter() {
            assert_eq!(process_part1(input), *idx);
        }
    }

    #[test]
    fn part2() {
        for (input, idx) in EXAMPLE_INPUTS_P2.iter() {
            assert_eq!(process_part2(input), *idx);
        }
    }
}
