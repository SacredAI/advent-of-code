use num::abs;

type Pairs = (Vec<isize>, Vec<isize>);

pub fn parse(input: &str) -> Pairs {
    let pairs: Pairs = input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            return (
                iter.next().unwrap().parse::<isize>().unwrap(),
                iter.next().unwrap().parse::<isize>().unwrap(),
            );
        })
        .unzip();
    let (mut left, mut right) = pairs;
    left.sort_unstable();
    right.sort_unstable();
    return (left, right);
}

pub fn part1(input: &Pairs) -> isize {
    input
        .0
        .iter()
        .zip(input.1.iter())
        .fold(0, |acc, (l, r)| abs(r - l) + acc)
}

pub fn part2(input: &Pairs) -> usize {
    input
        .0
        .iter()
        .map(|&l| input.1.iter().filter(|&r| *r == l).count() * (l as usize))
        .sum()
}
