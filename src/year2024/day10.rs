use crate::util::grid::Grid;
use crate::util::position::{Pos, ORTHOGONAL};
use std::collections::{HashMap, HashSet};

pub type Input = Grid<u8>;

pub fn parse(input: &str) -> Input {
    Grid::parse(input)
}

pub fn part1(input: &Input) -> u32 {
    let mut total = 0;
    for y in 0..input.height {
        for x in 0..input.width {
            let pos = Pos::new(x, y);
            if input[pos] != b'0' {
                continue;
            }
            let mut stack = vec![pos];
            let mut seen = HashSet::new();
            seen.insert(pos);
            let mut trail_nines = HashSet::new();
            while let Some(p) = stack.pop() {
                let v = input[p] - b'0';
                if v == 9 {
                    trail_nines.insert(p);
                }
                for d in &ORTHOGONAL {
                    let np = p + *d;
                    if !input.contains(np) {
                        continue;
                    }
                    if seen.contains(&np) {
                        continue;
                    }
                    let nv = input[np] - b'0';
                    if nv == v + 1 {
                        seen.insert(np);
                        stack.push(np);
                    }
                }
            }
            total += trail_nines.len() as u32;
        }
    }
    total
}

// New helper function to count distinct paths starting at a given position.
fn count_paths(pos: Pos, input: &Input, memo: &mut HashMap<Pos, u32>) -> u32 {
    if let Some(&cached) = memo.get(&pos) {
        return cached;
    }
    let v = input[pos] - b'0';
    if v == 9 {
        return 1;
    }
    let mut count = 0;
    for d in &ORTHOGONAL {
        let np = pos + *d;
        if !input.contains(np) {
            continue;
        }
        let nv = input[np] - b'0';
        if nv == v + 1 {
            count += count_paths(np, input, memo);
        }
    }
    memo.insert(pos, count);
    count
}

pub fn part2(input: &Input) -> u32 {
    let mut total = 0;
    let mut memo = HashMap::new();
    for y in 0..input.height {
        for x in 0..input.width {
            let pos = Pos::new(x, y);
            if input[pos] != b'0' {
                continue;
            }
            total += count_paths(pos, input, &mut memo);
        }
    }
    total
}
