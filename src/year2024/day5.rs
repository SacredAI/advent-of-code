use crate::util::parse::*;
use itertools::Itertools;
use std::cmp::Ordering::*;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let mut order = [[Greater; 100]; 100];

    for (from, to) in prefix.iter_unsigned::<usize>().tuples::<(_, _)>() {
        order[from][to] = Less;
    }

    let mut update = Vec::new();
    let mut part_one = 0;
    let mut part_two = 0;

    for line in suffix.lines() {
        update.clear();
        update.extend(line.iter_unsigned::<usize>());
        let middle = update.len() / 2;
        if update.is_sorted_by(|&from, &to| order[from][to] == Less) {
            part_one += update[middle];
        } else {
            update.select_nth_unstable_by(middle, |&from, &to| order[from][to]);
            part_two += update[middle]
        }
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}
