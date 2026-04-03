use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Machine {
    target: u16,
    buttons: Vec<u16>,
    joltage: Vec<u32>,
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(
            |line| match line.split(" ").collect::<Vec<&str>>().as_slice() {
                [target_str, buttons_str @ .., joltage_str] => {
                    let mut target: u16 = 0;
                    let mask: u16 = 0b_10000000_00000000;
                    for (shift, indicator) in target_str.as_bytes()[1..target_str.len() - 1]
                        .iter()
                        .enumerate()
                    {
                        if *indicator == b'#' {
                            target |= mask >> shift;
                        }
                    }

                    let mut buttons = Vec::new();

                    for button_str in buttons_str {
                        let mut button: u16 = 0;
                        for light in button_str[1..button_str.len() - 1]
                            .split(",")
                            .map(|light| light.parse::<u16>().unwrap())
                        {
                            button |= mask >> light;
                        }

                        buttons.push(button);
                    }

                    let joltage = joltage_str[1..joltage_str.len() - 1]
                        .split(",")
                        .map(|j| j.parse().unwrap())
                        .collect();

                    Machine {
                        target,
                        buttons,
                        joltage,
                    }
                }
                _ => unreachable!(),
            },
        )
        .collect()
}

// Key insight: you never need to press a button twice, since it'll just undo itself
// so we do.. smart bruteforce xd
// second key insight: the button algorithm is just xor'ing
#[aoc(day10, part1)]
fn part1(input: &[Machine]) -> String {
    todo!()
}

#[aoc(day10, part2)]
fn part2(input: &[Machine]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(part1(&parse(input)), "7");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
