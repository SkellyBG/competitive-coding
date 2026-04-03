use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::day8::union_find::UnionFind;

mod union_find;

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<JBox> {
    input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let mut it = line.split(',');
            (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
                idx,
            )
        })
        .collect()
}

type JBox = (u32, u32, u32, usize);

fn dist_squared(a: JBox, b: JBox) -> u64 {
    Into::<u64>::into(a.0.abs_diff(b.0)).pow(2)
        + Into::<u64>::into(a.1.abs_diff(b.1)).pow(2)
        + Into::<u64>::into(a.2.abs_diff(b.2)).pow(2)
}

#[aoc(day8, part1)]
fn part1(input: &[JBox]) -> String {
    let mut heap: BinaryHeap<Reverse<(u64, JBox, JBox)>> = BinaryHeap::new();

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let a = input[i];
            let b = input[j];
            heap.push(Reverse((dist_squared(a, b), a, b)));
        }
    }

    let mut uf = UnionFind::new(input.len());

    #[cfg(test)]
    let limit = 10;
    #[cfg(not(test))]
    let limit = 1000;

    for _ in 0..limit {
        let top = heap.pop().unwrap();
        uf.merge(top.0.1.3, top.0.2.3);
    }
    let mut sizes = uf.sizes();
    sizes.sort_unstable();
    sizes.into_iter().rev().take(3).product::<u64>().to_string()
}

#[aoc(day8, part2)]
fn part2(input: &[JBox]) -> String {
    let mut heap: BinaryHeap<Reverse<(u64, JBox, JBox)>> = BinaryHeap::new();

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let a = input[i];
            let b = input[j];
            heap.push(Reverse((dist_squared(a, b), a, b)));
        }
    }

    let mut uf = UnionFind::new(input.len());

    loop {
        let top = heap.pop().unwrap();
        uf.merge(top.0.1.3, top.0.2.3);
        if uf.sizes().len() == 1 {
            return (top.0.1.0 as u64 * top.0.2.0 as u64).to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(part1(&parse(input)), "40");
    }

    #[test]
    fn part2_example() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(part2(&parse(input)), "25272");
    }
}
