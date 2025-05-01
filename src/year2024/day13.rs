use itertools::Itertools;

use crate::util::parse::ParseOps;
type Input = Vec<(i64, i64, i64, i64, i64, i64)>;

pub fn parse(input: &str) -> Input {
    input.iter_signed().tuple_windows().collect()
}

pub fn part1(input: &Input) -> i64 {
    input
        .iter()
        .map(|(ax, ay, bx, by, mut px, mut py)| {
            let det = ax * by - ay * bx;
            if det == 0 {
                return 0;
            }

            let mut a = by * px - bx * py;
            let mut b = ax * py - ay * px;

            // Integer solutions only.
            if a % det != 0 || b % det != 0 {
                return 0;
            }

            a /= det;
            b /= det;

            3 * a + b
        })
        .sum()
}

pub fn part2(input: &Input) -> i64 {
    input
        .iter()
        .map(|(ax, ay, bx, by, mut px, mut py)| {
            px += 10000000000000;
            py += 10000000000000;
            let det = ax * by - ay * bx;
            if det == 0 {
                return 0;
            }

            let mut a = by * px - bx * py;
            let mut b = ax * py - ay * px;

            // Integer solutions only.
            if a % det != 0 || b % det != 0 {
                return 0;
            }

            a /= det;
            b /= det;

            3 * a + b
        })
        .sum()
}
