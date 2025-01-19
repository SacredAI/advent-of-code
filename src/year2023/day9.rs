pub fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split(" ")
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn n(rep: &[i64]) -> i64 {
    if rep.iter().all(|&i| i == 0) {
        return 0;
    }

    let m: Vec<i64> = rep.windows(2).map(|win| win[1] - win[0]).collect();

    return rep.last().unwrap() + n(&m);
}

pub fn part1(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|his| n(&his)).sum()
}

pub fn part2(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|his| {
            let mut new = his.clone();
            new.reverse();
            n(&new)
        })
        .sum()
}
