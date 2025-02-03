use crate::util::grid::Grid;
use crate::util::position::Pos;
use std::collections::{HashMap, HashSet};

type Input = Grid<u8>;

// Helper function: compute greatest common divisor.
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

pub fn parse(input: &str) -> Input {
    Grid::parse(input)
}

pub fn part1(input: &Input) -> u32 {
    // Collect antenna positions by frequency (non-empty cells).
    let mut antennas: HashMap<u8, Vec<Pos>> = HashMap::new();
    for y in 0..input.height {
        for x in 0..input.width {
            let pos = Pos::new(x, y);
            let cell = input[pos];
            if cell != b'.' {
                antennas.entry(cell).or_default().push(pos);
            }
        }
    }

    // Compute antinodes from each pair of antennas of the same frequency.
    let mut antinodes: HashSet<Pos> = HashSet::new();
    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let a = positions[i];
                let b = positions[j];
                // Candidate positions: X = 2*A - B and X = 2*B - A.
                let cand1 = Pos::new(2 * a.x - b.x, 2 * a.y - b.y);
                let cand2 = Pos::new(2 * b.x - a.x, 2 * b.y - a.y);
                if input.contains(cand1) {
                    antinodes.insert(cand1);
                }
                if input.contains(cand2) {
                    antinodes.insert(cand2);
                }
            }
        }
    }
    antinodes.len() as u32
}

pub fn part2(input: &Input) -> u32 {
    // Collect antenna positions by frequency (non-empty cells).
    let mut antennas: HashMap<u8, Vec<Pos>> = HashMap::new();
    for y in 0..input.height {
        for x in 0..input.width {
            let pos = Pos::new(x, y);
            let cell = input[pos];
            if cell != b'.' {
                antennas.entry(cell).or_default().push(pos);
            }
        }
    }
    let mut antinodes: HashSet<Pos> = HashSet::new();
    // For each frequency, consider every pair of antennas.
    for positions in antennas.values() {
        if positions.len() < 2 {
            continue;
        }
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let a = positions[i];
                let b = positions[j];
                let dx = b.x - a.x;
                let dy = b.y - a.y;
                let g = gcd(dx.abs(), dy.abs());
                // Get the minimal direction vector.
                let step_x = if g == 0 { 0 } else { dx / g };
                let step_y = if g == 0 { 0 } else { dy / g };

                // Find the starting point of the line in the negative direction.
                let mut start = a;
                loop {
                    let prev = Pos::new(start.x - step_x, start.y - step_y);
                    if input.contains(prev) {
                        start = prev;
                    } else {
                        break;
                    }
                }
                // Traverse along the line, marking all grid cells.
                let mut curr = start;
                while input.contains(curr) {
                    antinodes.insert(curr);
                    curr = Pos::new(curr.x + step_x, curr.y + step_y);
                }
            }
        }
    }
    antinodes.len() as u32
}
