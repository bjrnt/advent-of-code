use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

fn parse_droplets(input: &str) -> IResult<&str, Vec<(i32, i32, i32)>> {
    separated_list1(
        newline,
        separated_pair(
            complete::i32,
            tag(","),
            separated_pair(complete::i32, tag(","), complete::i32),
        )
        .map(|(x, (y, z))| (x, y, z)),
    )(input)
}

fn neighbors((x, y, z): &(i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
    [
        (*x + 1, *y, *z),
        (*x - 1, *y, *z),
        (*x, *y + 1, *z),
        (*x, *y - 1, *z),
        (*x, *y, *z + 1),
        (*x, *y, *z - 1),
    ]
    .into_iter()
}

pub fn process_part1(input: &str) -> String {
    let (_, droplets) = parse_droplets(input).unwrap();
    let space: HashSet<(i32, i32, i32)> = HashSet::from_iter(droplets.into_iter());

    let mut total_faces = 0;
    for droplet in space.iter() {
        let covered_faces = neighbors(droplet).filter(|n| space.contains(n)).count();
        dbg!(&droplet, &covered_faces);
        total_faces += 6 - covered_faces;
    }
    total_faces.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, droplets) = parse_droplets(input).unwrap();
    let droplet_count = droplets.len();
    let space: HashSet<(i32, i32, i32)> = HashSet::from_iter(droplets.into_iter());

    let mut total_faces = 0;
    for droplet in space.iter() {
        let mut covered_faces = 0;
        for neighbor in neighbors(droplet) {
            if space.contains(&neighbor) {
                covered_faces += 1;
            } else {
                // dfs to check if pocket of air is enclosed
                let mut visited = HashSet::new();
                let mut is_in_pocket = true;
                let mut stack = Vec::from([(neighbor, 0)]);
                while let Some((pos, depth)) = stack.pop() {
                    if depth > droplet_count / 2 {
                        is_in_pocket = false;
                        break;
                    }

                    visited.insert(pos);
                    for neighbor in
                        neighbors(&pos).filter(|n| !space.contains(n) && !visited.contains(n))
                    {
                        stack.push((neighbor, depth + 1));
                    }
                }

                if is_in_pocket {
                    covered_faces += 1;
                }
            }
        }
        total_faces += 6 - covered_faces;
    }
    total_faces.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "64");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "58");
    }
}
