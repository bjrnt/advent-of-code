use std::collections::HashMap;

const SHAPES: [&str; 5] = [
    "####",
    " #
###
 #",
    "  #
  #
###",
    "#
#
#
#",
    "##
##",
];

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Row(i64, i64);

fn parse_pushes(input: &str) -> Vec<i64> {
    input
        .chars()
        .map(|c| match c {
            '>' => 1,
            '<' => -1,
            _ => panic!("unrecognized char"),
        })
        .collect()
}

fn parse_shape(shape: &str) -> Vec<Row> {
    let mut rows: Vec<Row> = vec![];
    for (y, line) in shape.lines().rev().enumerate() {
        let mut row = 0;
        for (x, c) in line.char_indices().rev() {
            if c == '#' {
                row |= 1 << x;
            }
        }
        rows.push(Row(y as i64, row));
    }
    rows
}

// fn print_grid(grid: &HashMap<i64, i64>) {
//     let max_y = grid.keys().max().unwrap_or(&0);
//     for y in (0..=*max_y).rev() {
//         print!("{:2}: ", y);
//         let row = grid.get(&y).unwrap();
//         for x in 0..=8 {
//             if x == 0 || x == 8 {
//                 print!("@");
//             } else if (row & (1 << x)) != 0 {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         print!("\n");
//     }
//     print!("\n\n\n");
// }

fn shape_at(shape: &Vec<Row>, pos: (i64, i64)) -> impl Iterator<Item = Row> + '_ {
    shape
        .iter()
        .map(move |Row(y, row)| Row(y + pos.1, row << (pos.0 + 1)))
}

fn collides(a: i64, b: i64) -> bool {
    let walls = 1 | (1 << 8);
    (a & (b | walls)) != 0
}

fn process_shapes(input: &str, num_shapes: i64) -> String {
    let pushes = parse_pushes(input);

    let mut cache: HashMap<(usize, usize, Vec<i64>), (i64, i64)> = HashMap::new();

    let mut pushes = pushes.iter().enumerate().cycle();
    let mut shapes = SHAPES.into_iter().map(parse_shape).enumerate().cycle();
    let mut grid: Vec<i64> = vec![0];

    let mut max_y: i64 = 0;
    let mut max_y_offset: i64 = 0;
    let mut n_shapes = 0;

    while n_shapes < num_shapes {
        // print_grid(&grid);

        let (shape_idx, c_shape) = shapes.next().unwrap();
        let mut cur_pos = (2, max_y + 3);

        loop {
            let (push_idx, dx) = pushes.next().unwrap();
            let top_of_grid: Vec<_> = grid.iter().rev().take(100).map(|v| *v).collect();
            let cache_key = (shape_idx, push_idx, top_of_grid);
            let cached = cache.get(&cache_key);
            if cached.is_none() {
                cache.insert(cache_key, (max_y, n_shapes));
            } else {
                let (cached_max_y, cached_n_shapes) = cache.get(&cache_key).unwrap();
                let cycle_shapes = n_shapes - cached_n_shapes;
                let cycle_height = max_y - cached_max_y;
                let num_cycles = (num_shapes - n_shapes) / cycle_shapes;
                max_y_offset += num_cycles * cycle_height;
                n_shapes += num_cycles * cycle_shapes;
                // cache should only be used once, can't be bothered to spend more time on this problem so just clearing it
                cache.clear();
            }

            // check walls after push
            let next_pos = (cur_pos.0 + dx, cur_pos.1);
            let mut c_shape_coords = shape_at(&c_shape, next_pos);
            if c_shape_coords.all(|row| !collides(row.1, *grid.get(row.0 as usize).unwrap_or(&0))) {
                cur_pos = next_pos;
            }

            // fall down
            let next_pos = (cur_pos.0, cur_pos.1 - 1);
            let mut n_shape_coords = shape_at(&c_shape, next_pos);
            if n_shape_coords
                .any(|row| row.0 < 0 || (row.1 & *grid.get(row.0 as usize).unwrap_or(&0)) != 0)
            {
                // stop here
                let cur_coords = shape_at(&c_shape, cur_pos);

                for Row(y, row) in cur_coords {
                    max_y = max_y.max(y + 1);

                    // update grid
                    let mut grid_row = grid.get_mut(y as usize);
                    if grid_row.is_none() {
                        grid.push(0);
                        grid_row = grid.get_mut(y as usize);
                    }
                    let g = grid_row.unwrap();
                    *g = (*g) | row;
                }

                break;
            } else {
                // keep falling
                cur_pos = next_pos;
            }
        }

        n_shapes += 1;
    }

    (max_y + max_y_offset).to_string()
}

pub fn process_part1(input: &str) -> String {
    process_shapes(input, 2022)
}

pub fn process_part2(input: &str) -> String {
    process_shapes(input, 1_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const EXAMPLE_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "3068");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "1514285714288");
    }

    #[test]
    fn parse_shapes() {
        assert_eq!(parse_shape("####"), vec![Row(0, 15)]);
        assert_eq!(
            parse_shape(" #\n###\n #"),
            vec![Row(0, 2), Row(1, 1 + 2 + 4), Row(2, 2)]
        );
        assert_eq!(
            parse_shape("  #\n  #\n###"),
            vec![Row(0, 1 + 2 + 4), Row(1, 4), Row(2, 4)]
        )
    }

    #[test]
    fn move_shape() {
        let shape = parse_shape("####");
        let pos1 = (0, 0);
        assert_eq!(
            shape_at(&shape, pos1).collect_vec(),
            vec![Row(0, 2 + 4 + 8 + 16)]
        );
        let pos2 = (1, 0);
        assert_eq!(
            shape_at(&shape, pos2).collect_vec(),
            vec![Row(0, 4 + 8 + 16 + 32)]
        );
        let pos3 = (2, 0);
        assert_eq!(
            shape_at(&shape, pos3).collect_vec(),
            vec![Row(0, 8 + 16 + 32 + 64)]
        );
        let pos4 = (3, 0);
        assert_eq!(
            shape_at(&shape, pos4).collect_vec(),
            vec![Row(0, 16 + 32 + 64 + 128)]
        );
        let pos5 = (2, 1);
        assert_eq!(
            shape_at(&shape, pos5).collect_vec(),
            vec![Row(1, 8 + 16 + 32 + 64)]
        );
    }

    #[test]
    fn shape_collides() {
        let shape = parse_shape("####");
        // walls
        let moved_shape = shape_at(&shape, (0, 0)).collect_vec();
        assert_eq!(collides(moved_shape.get(0).unwrap().1, 0), false);
        let moved_shape = shape_at(&shape, (-1, 0)).collect_vec();
        assert_eq!(collides(moved_shape.get(0).unwrap().1, 0), true);
        let moved_shape = shape_at(&shape, (4, 0)).collect_vec();
        assert_eq!(collides(moved_shape.get(0).unwrap().1, 0), true);

        // other shapes
        let moved_shape = shape_at(&shape, (0, 0)).collect_vec();
        assert_eq!(collides(moved_shape.get(0).unwrap().1, 2), true);
        assert_eq!(collides(moved_shape.get(0).unwrap().1, 4), true);
        assert_eq!(collides(moved_shape.get(0).unwrap().1, 8), true);
        assert_eq!(collides(moved_shape.get(0).unwrap().1, 16), true);
        assert_eq!(collides(moved_shape.get(0).unwrap().1, 32), false);
    }
}
