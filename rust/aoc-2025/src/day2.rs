use aoc_runner_derive::{aoc, aoc_generator};
use fancy_regex::Regex;
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(u64, u64)]) -> String {
    let is_invalid = |num: u64| {
        let num_of_digit = num.ilog10() + 1;

        if num_of_digit % 2 == 1 {
            false
        } else {
            let top = num / 10_u64.pow(num_of_digit / 2);
            let bottom = num % 10_u64.pow(num_of_digit / 2);
            top == bottom
        }
    };

    input
        .iter()
        .map(|(start, end)| {
            (*start..=*end)
                .filter(|num: &u64| is_invalid(*num))
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
}

#[aoc(day2, part2)]
fn part2(input: &[(u64, u64)]) -> String {
    let re = Regex::new(r"^(\d+)\1{1,}$").unwrap();
    let is_invalid = |num: u64| re.is_match(&num.to_string());

    input
        .iter()
        .map(|(start, end)| {
            (*start..=*end)
                .filter(|num: &u64| is_invalid(*num).unwrap())
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(&parse(input)), "1227775554");
    }

    #[test]
    fn part2_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        // let input = "11-22";
        assert_eq!(part2(&parse(input)), "4174379265");
    }
}
