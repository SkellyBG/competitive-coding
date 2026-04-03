use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Vec<String>> {
    let input = input.to_string();

    let mut lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    for col in 0..lines[0].len() {
        if lines.iter().all(|line| line.as_bytes()[col] == b' ') {
            lines
                .iter_mut()
                .for_each(|line| unsafe { line.as_bytes_mut()[col] = b'|' });
        }
    }

    let mut result: Vec<Vec<String>> = Vec::new();

    for line in lines {
        for (idx, block) in line.split("|").enumerate() {
            match result.get_mut(idx) {
                Some(vec) => vec.push(block.to_string()),
                None => result.push(vec![block.to_string()]),
            }
        }
    }

    result
}

#[aoc(day6, part1)]
fn part1(input: &[Vec<String>]) -> String {
    input
        .iter()
        .map(|calc| {
            let mut nums: Vec<u64> = Vec::new();
            for num in calc.iter().rev().skip(1).rev() {
                nums.push(num.trim().parse().unwrap());
            }
            match calc.last().unwrap().trim().as_bytes()[0] {
                b'+' => nums.iter().sum::<u64>(),
                b'*' => nums.iter().product(),
                _ => unreachable!(),
            }
        })
        .sum::<u64>()
        .to_string()
}

#[aoc(day6, part2)]
fn part2(input: &[Vec<String>]) -> String {
    input
        .iter()
        .map(|calc| {
            let mut nums: Vec<u64> = Vec::new();
            for col in 0..calc[0].len() {
                let mut cur = String::new();
                for num in calc.iter().rev().skip(1).rev() {
                    cur.push(num.chars().nth(col).unwrap());
                }
                nums.push(cur.trim().parse().unwrap());
            }

            match calc.last().unwrap().trim().as_bytes()[0] {
                b'+' => nums.iter().sum::<u64>(),
                b'*' => nums.iter().product(),
                _ => unreachable!(),
            }
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(part1(&parse(input)), "4277556");
    }

    #[test]
    fn part2_example() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(part2(&parse(input)), "3263827");
    }
}
