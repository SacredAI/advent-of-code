use std::str::FromStr;

type Reveal = [u32; 3]; // r, g, b
pub struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

impl FromStr for Game {
    type Err = std::convert::Infallible;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id, rs) = line.split_once(':').unwrap();
        Ok(Self {
            id: id.split_once(' ').unwrap().1.parse().unwrap(),
            reveals: rs
                .split("; ")
                .map(|r| {
                    let mut reveal = [0, 0, 0];
                    for (n, color) in r.split(',').map(|s| s.trim().split_once(' ').unwrap()) {
                        reveal[color.as_bytes()[0] as usize % 3] = n.parse().unwrap();
                    }
                    reveal
                })
                .collect(),
        })
    }
}

pub fn parse(input: &str) -> Vec<Game> {
    input.lines().map(|g| Game::from_str(g).unwrap()).collect()
}

pub fn part1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter(|g| {
            g.reveals
                .iter()
                .all(|r| r[0] <= 12 && r[1] <= 13 && r[2] <= 14)
        })
        .map(|g| g.id)
        .sum()
}

pub fn part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|g| {
            (0..3)
                .map(|i| g.reveals.iter().map(|r| r[i]).max().unwrap())
                .product::<u32>()
        })
        .sum()
}
