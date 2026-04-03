use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[String]) -> String {
    let mut beams: HashSet<usize> = HashSet::new();
    let mut count = 0;

    for (idx, char) in input[0].as_bytes().iter().enumerate() {
        if *char == b'S' {
            beams.insert(idx);
        }
    }

    for line in input.iter().skip(1) {
        let mut new_set = HashSet::new();
        for (idx, char) in line.as_bytes().iter().enumerate() {
            if beams.contains(&idx) {
                if *char == b'^' {
                    new_set.insert(idx - 1);
                    new_set.insert(idx + 1);
                    count += 1;
                } else {
                    new_set.insert(idx);
                }
            }
        }
        // dbg!(beams);
        beams = new_set;
    }

    count.to_string()
}

#[aoc(day7, part2)]
fn part2(input: &[String]) -> String {
    let mut memo: HashMap<(usize, usize), i64> = HashMap::new();
    let mut starting_col = 0;
    for (idx, char) in input[0].as_bytes().iter().enumerate() {
        if *char == b'S' {
            starting_col = idx
        }
    }

    recurse(1, starting_col, input, &mut memo).to_string()
}

fn recurse(
    row: usize,
    col: usize,
    input: &[String],
    memo: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    if let Some(value) = memo.get(&(row, col)) {
        return *value;
    }

    if row == input.len() - 1 {
        return 1;
    }

    let value = match input[row + 1].as_bytes()[col] {
        b'.' => recurse(row + 1, col, input, memo),
        b'^' => recurse(row + 1, col - 1, input, memo) + recurse(row + 1, col + 1, input, memo),
        _ => unreachable!(),
    };

    memo.insert((row, col), value);
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part1(&parse(input)), "21");
    }

    #[test]
    fn part2_example() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part2(&parse(input)), "40");
    }
}
