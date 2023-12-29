use itertools::Itertools;

pub struct Game {
    times: Vec<u32>,
    dist: Vec<u32>,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Game {
    let mut parts = input.lines();
    Game {
        times: parts
            .next()
            .unwrap()
            .replace("Time:", "")
            .trim()
            .split(' ')
            .filter_map(|f| f.trim().parse::<u32>().ok())
            .collect::<Vec<u32>>(),
        dist: parts
            .next()
            .unwrap()
            .replace("Distance:", "")
            .trim()
            .split(' ')
            .filter_map(|f| f.trim().parse::<u32>().ok())
            .collect::<Vec<u32>>(),
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &Game) -> u32 {
    let counts = input
        .times
        .iter()
        .enumerate()
        .map(|(i, &time)| {
            (1..time).fold(0, |count, hold_time| {
                match (hold_time * (time - hold_time)) > *input.dist.get(i).unwrap() {
                    true => count + 1,
                    false => count,
                }
            })
        })
        .collect::<Vec<u32>>();
    counts.iter().product::<u32>()
}

#[aoc(day6, part2)]
pub fn part2(input: &Game) -> u64 {
    let time = input.times.iter().join("").parse::<u64>().unwrap();
    let dist = input.dist.iter().join("").parse::<u64>().unwrap();

    (1..time).fold(0, |count, hold_time| {
        match (hold_time * (time - hold_time)) > dist {
            true => count + 1,
            false => count,
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::day6::*;
    #[test]
    fn p1() {
        let i = input_generator(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        let result = part1(&i);
        assert_eq!(result, 288);
    }

    #[test]
    fn p2() {
        let mut i = input_generator(
            "Time:      7 15 30
Distance:  9 40 200",
        );
        let result = part2(&mut i);
        assert_eq!(result, 71503);
    }
}
