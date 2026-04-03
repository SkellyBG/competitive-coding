use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().iter().map(|c| *c - b'0').collect())
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u8>]) -> String {
    input
        .iter()
        .map(|line| {
            let max = line.iter().max().unwrap();
            let max_idx = line.iter().position(|x| x == max).unwrap();

            if max_idx == line.len() - 1 {
                let second_max = line.iter().filter(|x| *x != max).max().unwrap();
                (second_max * 10 + max) as u64
            } else {
                let second_max = line[max_idx + 1..].iter().max().unwrap();
                (max * 10 + second_max) as u64
            }
        })
        .sum::<u64>()
        .to_string()
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u8>]) -> String {
    // greedy algorithm - we take all the last digits, and then tries to maximise each digit from highest to lowest by moving them forward
    const BATTERY_COUNT: usize = 12;
    input
        .iter()
        .map(|line| {
            let mut sum: u64 = 0;
            let mut prev_idx = -1;
            for offset_from_back in (0..BATTERY_COUNT).rev() {
                let mut cur_idx = line.len() - 1 - offset_from_back;
                let mut best_idx = cur_idx;
                let mut best = line[cur_idx];
                while cur_idx as i32 > prev_idx {
                    if best <= line[cur_idx] {
                        best = line[cur_idx];
                        best_idx = cur_idx;
                    }
                    if cur_idx == 0 {
                        break;
                    }
                    cur_idx -= 1;
                }

                prev_idx = best_idx as i32;
                sum *= 10;
                sum += best as u64;
            }

            sum
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part1(&parse(input)), "357");
    }

    #[test]
    fn part2_example() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part2(&parse(input)), "3121910778619");
    }
}
