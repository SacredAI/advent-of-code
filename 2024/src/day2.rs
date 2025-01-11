use itertools::Itertools;

enum Direction {
    Decreasing,
    Increasing,
}

fn check_row(row: &[isize]) -> Result<(), String> {
    let mut dir: Option<Direction> = None;
    for (a, b) in row.iter().tuple_windows() {
        let diff = a - b;
        if !(1..=3).contains(&diff.abs()) {
            return Err(format!("{}, {} diff value {}", a, b, diff));
        }
        match diff.signum() {
            -1 => match dir {
                Some(Direction::Increasing) => {
                    return Err(format!("Direction changed to Increasing"))
                }
                Some(Direction::Decreasing) => {
                    continue;
                }
                None => {
                    dir = Some(Direction::Decreasing);
                }
            },
            1 => match dir {
                Some(Direction::Increasing) => {
                    continue;
                }
                Some(Direction::Decreasing) => {
                    return Err(format!("Direction Changed to Decreasing"))
                }
                None => {
                    dir = Some(Direction::Increasing);
                }
            },
            _ => return Err(format!("Should have a difference")),
        }
    }
    Ok(())
}

#[aoc_generator(day2)]
pub fn preprocess(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[aoc(day2, part1)]
pub fn part1(rows: &[Vec<isize>]) -> usize {
    rows.iter().filter(|row| check_row(&row).is_ok()).count()
}

#[aoc(day2, part2)]
pub fn part2(rows: &[Vec<isize>]) -> usize {
    rows.iter()
        .filter(|row| {
            if check_row(&row).is_err() {
                for index in 0..row.len() {
                    let mut new_report = (*row).clone();
                    new_report.remove(index);
                    if check_row(&new_report).is_ok() {
                        return true;
                    }
                }
                return false;
            }
            return true;
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day2::part1;

    #[test]
    fn p1() {
        const input: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert!(part1(input) == 2)
    }

    #[test]
    fn p2() {}
}
