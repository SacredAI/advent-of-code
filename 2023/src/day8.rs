use num::integer::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Node<'a> {
    left: &'a str,
    right: &'a str,
    key: &'a str,
}

impl<'a> Node<'a> {
    fn from_str(line: &'a str) -> Self {
        let (key, left_right) = line.split_once(" = ").unwrap();
        let (left, right) = left_right[1..left_right.len() - 1]
            .split_once(", ")
            .unwrap();
        Self { key, left, right }
    }
}

pub type Network<'a> = HashMap<&'a str, Node<'a>>;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    LEFT,
    RIGHT,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::LEFT,
            'R' => Direction::RIGHT,
            _ => panic!("Invalid Direction"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game<'a> {
    dirs: Vec<Direction>,
    net: Network<'a>,
}

// #[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Game<'_> {
    let (dirs, network) = input.split_once("\n\n").unwrap();
    let directions: Vec<Direction> = dirs.chars().map(Direction::from).collect();
    let net = network.lines().fold(Network::new(), |mut acc, line| {
        let n = Node::from_str(line);
        acc.insert(n.key, n);
        acc
    });
    Game {
        dirs: directions,
        net: net,
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u32 {
    let game = input_generator(input);
    let mut steps = 0;
    let mut node = "AAA";
    let mut dirs = game.dirs.iter().cycle();

    while node != "ZZZ" {
        match dirs.next().unwrap() {
            Direction::LEFT => node = game.net[node].left,
            Direction::RIGHT => node = game.net[node].right,
        }
        steps += 1;
    }
    steps
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u64 {
    let game = input_generator(input);
    let keys = game.net.clone().into_iter();

    let mut result = 1;

    for (key, _value) in keys {
        let mut node = key;
        if node.ends_with("A") {
            let mut steps = 0;
            let clone_dirs = game.dirs.clone();
            let mut dirs = clone_dirs.iter().cycle();

            while !node.ends_with("Z") {
                match dirs.next().unwrap() {
                    Direction::LEFT => node = game.net[node].left,
                    Direction::RIGHT => node = game.net[node].right,
                }
                steps += 1;
            }

            result = result.lcm(&steps);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day8::*;
    #[test]
    fn p1() {
        let result = part1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 6);
    }

    #[test]
    fn p2() {
        let result = part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, 6);
    }
}
