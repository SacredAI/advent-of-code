use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tile {
    ROUND,
    CUBE,
    EMPTY,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::CUBE,
            'O' => Self::ROUND,
            '.' => Self::EMPTY,
            _ => unreachable!("AOC only provides valid input"),
        }
    }
}

impl Tile {
    #[inline]
    const fn is_round(self) -> bool {
        matches!(self, Self::ROUND)
    }

    #[inline]
    const fn is_empty(self) -> bool {
        matches!(self, Self::EMPTY)
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    data: Vec<Vec<Tile>>,
    nrows: usize,
    ncols: usize,
}

impl FromStr for Grid {
    type Err = std::convert::Infallible;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let grid = input
            .lines()
            .map(|l| l.chars().map(Tile::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Self {
            nrows: grid[0].len(),
            ncols: grid.len(),
            data: grid,
        })
    }
}

impl Grid {
    fn total_load(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|cell| cell.is_round()).count())
            .rev()
            .enumerate()
            .map(|(idx, count)| (idx + 1) * count)
            .sum()
    }

    /// Encode the locations of the rounded rocks, assuming grid is 100x100 max.
    ///
    /// NOTE: `64 * 157 > 100 * 100`
    fn id(&self) -> [u64; 157] {
        let mut bits = [0; 157];
        for (idx, obj) in self.data.iter().flatten().enumerate() {
            if obj.is_round() {
                bits[idx / 64] |= 1 << (idx % 64);
            }
        }
        bits
    }

    fn roll_north(&mut self) {
        self.roll_helper(
            (1..self.nrows).cartesian_product(0..self.ncols),
            |(r, c)| (0..r).rev().map(move |i| (i, c)),
        );
    }

    fn roll_south(&mut self) {
        let nrows = self.nrows;
        self.roll_helper(
            (0..nrows - 1).rev().cartesian_product(0..self.ncols),
            |(r, c)| (r + 1..nrows).map(move |i| (i, c)),
        );
    }

    fn roll_west(&mut self) {
        self.roll_helper(
            (1..self.ncols)
                .cartesian_product(0..self.nrows)
                .map(|(c, r)| (r, c)),
            |(r, c)| (0..c).rev().map(move |i| (r, i)),
        );
    }

    fn roll_east(&mut self) {
        let ncols = self.ncols;
        self.roll_helper(
            (0..ncols - 1)
                .rev()
                .cartesian_product(0..self.nrows)
                .map(|(c, r)| (r, c)),
            |(r, c)| (c + 1..ncols).map(move |i| (r, i)),
        );
    }

    #[inline]
    fn roll_cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    #[inline]
    fn roll_helper<C, R>(&mut self, coords: C, ray: impl Fn((usize, usize)) -> R)
    where
        C: Iterator<Item = (usize, usize)>,
        R: Iterator<Item = (usize, usize)>,
    {
        for (r, c) in coords {
            if self.data[r][c].is_round() {
                let empties = ray((r, c)).take_while(|&(r0, c0)| self.data[r0][c0].is_empty());
                if let Some((r1, c1)) = empties.last() {
                    self.data[r][c] = Tile::EMPTY;
                    self.data[r1][c1] = Tile::ROUND;
                }
            }
        }
    }
}

// Useful to visually check the example grid after cycles 1 2 3.
impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for obj in row {
                match obj {
                    Tile::EMPTY => f.write_str(".")?,
                    Tile::ROUND => f.write_str("O")?,
                    Tile::CUBE => f.write_str("#")?,
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();
    grid.roll_north();
    grid.total_load()
}

pub fn part2(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();
    const NB_CYCLES: u32 = 1_000_000_000;
    let mut cache = HashMap::with_capacity(150);
    for step in 0..NB_CYCLES {
        grid.roll_cycle();
        if let Some(prev_step) = cache.insert(grid.id(), step) {
            let remaining = NB_CYCLES - 1 - step;
            let period = step - prev_step;
            for _ in 0..remaining % period {
                grid.roll_cycle();
            }
            break;
        }
    }
    grid.total_load()
}
