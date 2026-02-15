#[derive(Clone)]
enum Square {
    Empty,
    Filled,
}

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<Square>> {
    input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|char| match char {
                    b'.' => Square::Empty,
                    b'@' => Square::Filled,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Vec<Square>]) -> String {
    let dirs = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];
    let mut total = 0;
    for (r_idx, row) in input.iter().enumerate() {
        for (c_idx, square) in row.iter().enumerate() {
            match square {
                Square::Empty => continue,
                Square::Filled => {
                    let mut around = 0;
                    for dir in dirs {
                        let new_r = r_idx as i32 - dir.0;
                        let new_c = c_idx as i32 - dir.1;

                        if new_r < 0
                            || new_c < 0
                            || new_r >= input.len() as i32
                            || new_c >= row.len() as i32
                        {
                            continue;
                        }

                        if matches!(input[new_r as usize][new_c as usize], Square::Filled) {
                            around += 1;
                        }
                    }
                    if around < 4 {
                        total += 1;
                    }
                }
            }
        }
    }

    total.to_string()
}

#[aoc(day4, part2)]
fn part2(input: &[Vec<Square>]) -> String {
    let mut input = input.to_vec();
    let dirs = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];
    let mut total = 0;

    loop {
        let existings = {
            let mut existings = Vec::new();
            for (r_idx, row) in input.iter().enumerate() {
                for (c_idx, square) in row.iter().enumerate() {
                    match square {
                        Square::Empty => continue,
                        Square::Filled => {
                            let mut around = 0;
                            for dir in dirs {
                                let new_r = r_idx as i32 - dir.0;
                                let new_c = c_idx as i32 - dir.1;

                                if new_r < 0
                                    || new_c < 0
                                    || new_r >= input.len() as i32
                                    || new_c >= row.len() as i32
                                {
                                    continue;
                                }

                                if matches!(input[new_r as usize][new_c as usize], Square::Filled) {
                                    around += 1;
                                }
                            }
                            if around < 4 {
                                existings.push((r_idx, c_idx));
                            }
                        }
                    }
                }
            }

            total += existings.len();

            existings
        };

        if existings.is_empty() {
            break;
        }

        for existing in existings {
            input[existing.0][existing.1] = Square::Empty;
        }
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part1(&parse(input)), "13");
    }

    #[test]
    fn part2_example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part2(&parse(input)), "43");
    }
}
