use itertools::Itertools;

pub struct Day11<'a> {
    universe: Vec<&'a [u8]>,
    galaxies: Vec<(usize, usize)>,
}

impl Day11<'_> {
    fn solve(&mut self, galaxy_size: usize) -> usize {
        let (rows, cols) = (self.universe.len(), self.universe[0].len());
        let empty_rows = (0..rows).filter(|&r| self.universe[r].iter().all(|&c| c == b'.'));
        let empty_cols = (0..cols).filter(|&c| (0..rows).all(|r| self.universe[r][c] == b'.'));
        for r in empty_rows.rev() {
            for g in &mut self.galaxies {
                if g.0 > r {
                    g.0 += galaxy_size - 1
                }
            }
        }
        for c in empty_cols.rev() {
            for g in &mut self.galaxies {
                if g.1 > c {
                    g.1 += galaxy_size - 1
                }
            }
        }
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(&(r1, c1), &(r2, c2))| r1.abs_diff(r2) + c1.abs_diff(c2))
            .sum()
    }
}

// #[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Day11<'_> {
    let universe = input.split("\n").map(|s| s.as_bytes()).collect::<Vec<_>>();
    let galaxies = (0..universe.len())
        .cartesian_product(0..universe[0].len())
        .filter(|&(r, c)| universe[r][c] == b'#')
        .collect::<Vec<_>>();
    Day11 { universe, galaxies }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let mut day = input_generator(input);
    day.solve(2)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let mut day = input_generator(input);
    day.solve(1000000)
}
