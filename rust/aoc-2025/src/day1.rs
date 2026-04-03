use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day1)]
fn parse(input: &str) -> String {
    input.to_string()
}

enum Direction {
    Left,
    Right,
}

#[aoc(day1, part1)]
fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| match line.as_bytes() {
            [dir, rest @ ..] => (
                if *dir == b'L' {
                    Direction::Left
                } else {
                    Direction::Right
                },
                str::from_utf8(rest).unwrap().parse::<i32>().unwrap(),
            ),
            _ => unreachable!(),
        })
        .fold((50, 0), |(mut cur, mut total), op| {
            match op.0 {
                Direction::Right => cur = (cur + op.1) % 100,
                Direction::Left => cur = ((cur - op.1) % 100 + 100) % 100,
            }

            if cur == 0 {
                total += 1;
            }
            (cur, total)
        })
        .1
        .to_string()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| match line.as_bytes() {
            [dir, rest @ ..] => (
                match dir {
                    b'L' => Direction::Left,
                    b'R' => Direction::Right,
                    _ => unreachable!(),
                },
                str::from_utf8(rest).unwrap().parse::<i32>().unwrap(),
            ),
            _ => unreachable!(),
        })
        .fold((50, 0), |(mut cur, total), op| {
            assert!(cur >= 0);
            assert!(cur < 100);

            match op.0 {
                Direction::Right => {
                    cur += op.1;

                    let mut inc = 0;
                    while cur >= 100 {
                        cur -= 100;
                        inc += 1;
                    }

                    (cur, total + inc)
                }
                Direction::Left => {
                    let mut inc = 0;
                    for _ in 0..op.1 {
                        cur -= 1;
                        if cur == 0 || cur == -100 {
                            inc += 1;
                            cur = 0;
                        }
                    }

                    (cur.rem_euclid(100), total + inc)
                }
            }
        })
        .1
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part1(&parse(input)), "3");
    }

    #[test]
    fn part2_example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part2(&parse(input)), "6");
    }
}
