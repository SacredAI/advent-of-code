use std::{str::FromStr, collections::HashSet};

pub struct Game {
    id: u32,
    wins: Vec<u32>,
    extra_copies: u32,
}

impl FromStr for Game {
    type Err = std::convert::Infallible;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id, rs) = line.split_once(':').unwrap();
        let (ws, ns) = rs.split_once("|").unwrap();

        let winning_nums: HashSet<u32> = ws
        .split(" ")
        .filter_map(|r| r.trim().parse::<u32>().ok())
        .collect();

        let player_nums: HashSet<u32> = ns
        .split(" ")
        .filter_map(|r| r.trim().parse::<u32>().ok())
        .collect();

        let inter_a = winning_nums.intersection(&player_nums).collect::<Vec<_>>();

        Ok(Self {
            id: id.split_once(' ').unwrap().1.trim().parse().unwrap(),
            wins: inter_a.iter().map(|&&x| x).collect(),
            extra_copies: 0
        })
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map(|g| Game::from_str(g).unwrap()).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Game]) -> u32 {

    input
        .iter()
        .map(|g| match g.wins.len(){
            0 => 0,
            x => u32::pow(2, (x - 1) as u32),
        })
        .sum::<u32>()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Game]) -> u32 {
    let mut amount = vec![1; input.len()];
    input
        .iter()
        .enumerate()
        .for_each(|(i, g)| {
            let wins = g.wins.len();
            let cur_amount = amount[i];
            for offset in 1..wins+1 {
                amount[i + offset] += cur_amount;
            }
        });
    amount.iter().sum()
}

#[cfg(test)]
mod tests{
    use crate::day4::*;
    #[test]
    fn p1(){
        let i = input_generator("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        let result = part1(&i);
        assert_eq!(result, 13);
    }

    #[test]
    fn p2(){
        let mut i = input_generator("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        let result = part2(&mut i);
        assert_eq!(result, 30);
    }
}
