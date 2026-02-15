use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    for line in input.lines() {
        match line.split_once("-") {
            Some((left, right)) => ranges.push(left.parse().unwrap()..=right.parse().unwrap()),
            None => {
                if line.is_empty() {
                    continue;
                }

                ingredients.push(line.parse().unwrap());
            }
        }
    }

    (ranges, ingredients)
}

#[aoc(day5, part1)]
fn part1(input: &(Vec<RangeInclusive<u64>>, Vec<u64>)) -> String {
    let (ranges, ingredients) = input;

    ingredients
        .iter()
        .filter(|ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .collect::<Vec<_>>()
        .len()
        .to_string()
}

#[aoc(day5, part2)]
fn part2(input: &(Vec<RangeInclusive<u64>>, Vec<u64>)) -> String {
    let (mut ranges, _) = input.clone();

    let merged_range = {
        ranges.sort_by(|a, b| a.start().cmp(b.start()));

        let mut merged_range: Vec<RangeInclusive<u64>> = Vec::new();

        for range in ranges {
            if let Some(last_range) = merged_range.last()
                && last_range.end() >= range.start()
            {
                let last_range = merged_range.pop().unwrap();
                merged_range.push(*last_range.start()..=*last_range.end().max(range.end()))
            } else {
                merged_range.push(range)
            }
        }

        merged_range
    };

    merged_range
        .into_iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(part1(&parse(input)), "3");
    }

    #[test]
    fn part2_example() {
        let input = "3-5
10-14
16-20
12-18
";
        assert_eq!(part2(&parse(input)), "14");
    }
}
