#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add(u32),
    Remove,
}

use Op::*;

fn parse_step(input: &str) -> (&str, Op) {
    let label_ends_at = input.chars().position(|c| c == '=' || c == '-').unwrap();
    let label = &input[..label_ends_at];
    let op = match input.chars().nth(label_ends_at).unwrap() {
        '=' => Add(input
            .chars()
            .nth(label_ends_at + 1)
            .unwrap()
            .to_digit(10)
            .unwrap()),
        '-' => Remove,
        _ => unreachable!(),
    };
    (label, op)
}

pub fn process_part1(input: &str) -> String {
    input.split(',').map(hash).sum::<usize>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    for (label, op) in input.split(',').map(parse_step) {
        let box_idx = hash(label);
        let lens_idx = boxes[box_idx]
            .iter()
            .position(|(label2, _)| label == *label2);
        match (op, lens_idx) {
            (Add(lens), Some(idx)) => boxes[box_idx][idx].1 = lens,
            (Add(lens), None) => boxes[box_idx].push((label.to_string(), lens)),
            (Remove, Some(idx)) => {
                boxes[box_idx].remove(idx);
            }
            (Remove, None) => (),
        };
    }
    boxes
        .into_iter()
        .enumerate()
        .map(|(box_idx, b)| {
            b.into_iter()
                .enumerate()
                .map(|(lens_idx, (_, lens))| (box_idx + 1) * (lens_idx + 1) * (lens as usize))
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |mut hash, c| {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
        hash
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", "1320")]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part1(input).as_str(), expected);
    }

    #[rstest]
    #[case("HASH", "52")]
    #[trace]
    fn test_hash(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(hash(input).to_string().as_str(), expected);
    }

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", "145")]
    #[trace]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part2(input).as_str(), expected);
    }
}
