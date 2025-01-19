use std::{collections::HashSet, str::FromStr};

pub struct Game {
    id: u32,
    wins: Vec<u32>,
    extra_copies: u32,
}

impl FromStr for Game {
    type Err = std::convert::Infallible;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id, rs) = line.split_once(':').unwrap();
        let (ws, ns) = rs.split_once('|').unwrap();

        let winning_nums: HashSet<u32> = ws
            .split(' ')
            .filter_map(|r| r.trim().parse::<u32>().ok())
            .collect();

        let player_nums: HashSet<u32> = ns
            .split(' ')
            .filter_map(|r| r.trim().parse::<u32>().ok())
            .collect();

        let inter_a = winning_nums.intersection(&player_nums).collect::<Vec<_>>();

        Ok(Self {
            id: id.split_once(' ').unwrap().1.trim().parse().unwrap(),
            wins: inter_a.iter().map(|&&x| x).collect(),
            extra_copies: 0,
        })
    }
}

pub fn parse(input: &str) -> Vec<Game> {
    input.lines().map(|g| Game::from_str(g).unwrap()).collect()
}

pub fn part1(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|g| match g.wins.len() {
            0 => 0,
            x => u32::pow(2, (x - 1) as u32),
        })
        .sum::<u32>()
}

pub fn part2(input: &[Game]) -> u32 {
    let mut amount = vec![1; input.len()];
    input.iter().enumerate().for_each(|(i, g)| {
        let wins = g.wins.len();
        let cur_amount = amount[i];
        for offset in 1..wins + 1 {
            amount[i + offset] += cur_amount;
        }
    });
    amount.iter().sum()
}
