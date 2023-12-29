#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<i64>> {
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

#[aoc(day9, part1)]
pub fn part1(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|his| n(&his)).sum()
}

#[aoc(day9, part2)]
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

#[cfg(test)]
mod tests {
    use crate::day9::*;
    #[test]
    fn p1() {
        let i = input_generator(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        let result = part1(&i);
        assert_eq!(result, 114);
    }

    #[test]
    fn p2() {
        let i = input_generator("");
        let result = part2(&i);
        assert_eq!(result, 6);
    }
}
