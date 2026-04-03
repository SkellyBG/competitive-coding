use std::cmp::{max, min};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            line.split_once(",")
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn rectangle_area(a: (u64, u64), b: (u64, u64)) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

#[aoc(day9, part1)]
fn part1(input: &[(u64, u64)]) -> String {
    let mut result = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            result = result.max(rectangle_area(input[i], input[j]))
        }
    }

    result.to_string()
}

// we need to pick two corner in a polygon, and check if the rectangle they form lies completely within the polygon
// the list is nicely given to us as a (clockwise/counter cw) polygon corners
// picking two corners, we know the rectangle lies within the polygon if none of the edge of the polygon intersect with the rectangle
// (though we need to be carefuly with handling cases)

// an edge "intersect with the rectangle" if a part of them lies strictly between an opposite edge pair of the rectangle
fn is_valid_rectangle(a: (u64, u64), b: (u64, u64), polygon_corners: &[(u64, u64)]) -> bool {
    for i in 0..polygon_corners.len() {
        let first_end = polygon_corners[i];
        let second_end = polygon_corners[(i + 1) % polygon_corners.len()];

        if first_end.0 == second_end.0 {
            let min_x = min(a.0, b.0);
            let max_x = max(a.0, b.0);

            if min_x < first_end.0 && first_end.0 < max_x {
                let min_y = min(a.1, b.1);
                let max_y = max(a.1, b.1);

                if !(first_end.1 >= max_y && second_end.1 >= max_y
                    || first_end.1 <= min_y && second_end.1 <= min_y)
                {
                    return false;
                }
            }
        } else if first_end.1 == second_end.1 {
            let min_y = min(a.1, b.1);
            let max_y = max(a.1, b.1);

            if min_y < first_end.1 && first_end.1 < max_y {
                let min_x = min(a.0, b.0);
                let max_x = max(a.0, b.0);

                if !(first_end.0 >= max_x && second_end.0 >= max_x
                    || first_end.0 <= min_x && second_end.0 <= min_x)
                {
                    return false;
                }
            }
        } else {
            unreachable!();
        }
    }

    true
}

#[aoc(day9, part2)]
fn part2(input: &[(u64, u64)]) -> String {
    let mut result = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let a = input[i];
            let b = input[j];

            if !is_valid_rectangle(a, b, input) {
                continue;
            }
            result = result.max(rectangle_area(a, b))
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part1(&parse(input)), "50");
    }

    #[test]
    fn part2_example() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part2(&parse(input)), "24");
    }
}
