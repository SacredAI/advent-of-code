use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Entry {
    None,
    Symbol(char),
    Digit(u32),
}

impl From<char> for Entry {
    fn from(value: char) -> Self {
        match value.to_digit(10) {
            Some(x) => Self::Digit(x),
            None => match value {
                '.' => Self::None,
                x => Self::Symbol(x),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Digit {
    pub start_col: usize,
    pub line: usize,
    pub len: usize,
    pub digit: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Symbol {
    pub col: usize,
    pub line: usize,
    pub symbol: char,
}

impl Digit {
    fn is_close(&self, symbol: &Symbol) -> bool {
        for i in 0..self.len {
            if (self.start_col + i).abs_diff(symbol.col) <= 1
                && self.line.abs_diff(symbol.line) <= 1
            {
                return true;
            }
        }
        false
    }
}

impl Symbol {
    pub(crate) fn get_numbers_close(self, pns: &[Digit]) -> impl Iterator<Item = Digit> + '_ {
        pns.iter().copied().filter(move |pn| pn.is_close(&self))
    }
}

#[derive(Debug)]
pub struct Schem {
    numbers: Vec<Digit>,
    symbols: Vec<Symbol>,
}

impl Schem {
    pub(crate) fn get_part_numbers(&self) -> impl Iterator<Item = Digit> + '_ {
        self.numbers
            .iter()
            .copied()
            .filter(|pn| self.symbols.iter().any(|s| pn.is_close(s)))
    }
}

impl FromStr for Schem {
    type Err = std::convert::Infallible;
    fn from_str(lines: &str) -> Result<Self, Self::Err> {
        let mut nums = vec![];
        let mut symbs = vec![];

        let columns = lines
            .lines()
            .next()
            .expect("File should not be empty")
            .len();

        let mut digits_buffer = Vec::with_capacity(columns);

        for (i, line) in lines.lines().enumerate() {
            let entries: Vec<Entry> = line.chars().map(Entry::from).collect();

            let pos_symbols = entries.iter().enumerate().filter_map(|(j, e)| {
                if let Entry::Symbol(x) = e {
                    Some(Symbol {
                        line: i,
                        col: j,
                        symbol: *x,
                    })
                } else {
                    None
                }
            });

            let grouped_digits = entries
                .iter()
                .copied()
                .enumerate()
                .group_by(|(_, e)| matches!(e, Entry::Digit(_)));

            let pos_numbers = grouped_digits.into_iter().flat_map(|(k, g)| {
                if !k {
                    None
                } else {
                    digits_buffer.extend(g);

                    let g = &digits_buffer;
                    let n = g.len();

                    let start_col = g[0].0;

                    let mut number = 0;
                    for (k, entry) in g.iter().enumerate() {
                        match entry.1 {
                            Entry::Digit(x) => {
                                number += 10_usize.pow((n - k - 1) as u32) * x as usize
                            }
                            _ => unreachable!("Groups have been filtered to only contain digits"),
                        }
                    }

                    digits_buffer.clear();

                    Some(Digit {
                        len: n,
                        start_col,
                        line: i,
                        digit: number as u32,
                    })
                }
            });

            nums.extend(pos_numbers);
            symbs.extend(pos_symbols);
        }

        Ok(Self {
            numbers: nums,
            symbols: symbs,
        })
    }
}

pub fn parse(input: &str) -> Schem {
    Schem::from_str(input).unwrap()
}

pub fn part1(input: &Schem) -> u32 {
    input.get_part_numbers().map(|pn| pn.digit).sum()
}

pub fn part2(input: &Schem) -> u32 {
    input
        .symbols
        .iter()
        .filter(|symbol| symbol.symbol == '*')
        .filter_map(|symbol| {
            let numbers_close = symbol.get_numbers_close(&input.numbers);

            if let Some((x, y)) = numbers_close.collect_tuple() {
                Some((x, y))
            } else {
                None
            }
        })
        .map(|numbers_close| numbers_close.0.digit * numbers_close.1.digit)
        .sum()
}
