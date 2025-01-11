use num::abs;

type Pairs = (Vec<isize>, Vec<isize>);

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Pairs {
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

#[aoc(day1, part1)]
pub fn part1(input: &Pairs) -> isize {
    input
        .0
        .iter()
        .zip(input.1.iter())
        .fold(0, |acc, (l, r)| abs(r - l) + acc)
}

#[aoc(day1, part2)]
pub fn part2(input: &Pairs) -> usize {
    input
        .0
        .iter()
        .map(|&l| input.1.iter().filter(|&r| *r == l).count() * (l as usize))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day1::{generator, part1};

    #[test]
    fn p1() {
        const test: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
        let input = generator(test);
        assert_eq!(11, part1(&input))
    }

    #[test]
    fn p2() {}
}
