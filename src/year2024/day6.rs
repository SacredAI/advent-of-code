use std::collections::HashSet;

use crate::util::grid::*;
use crate::util::position::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(base_grid: &Grid<u8>) -> usize {
    let mut grid = base_grid.clone();
    let mut position = grid.find(b'^').unwrap();
    let mut direction = UP;
    let mut result = 1;

    while grid.contains(position + direction) {
        if grid[position + direction] == b'#' {
            direction = direction.clockwise();
            continue;
        }

        let next = position + direction;

        // Avoid double counting when the path crosses itself.
        if grid[next] == b'.' {
            result += 1;
            grid[next] = b'^';
        }

        position = next;
    }

    result
}

pub fn part2(grid: &Grid<u8>) -> usize {
    let mut grid = grid.clone();
    let mut position = grid.find(b'^').unwrap();
    let mut direction = UP;
    let mut path = Vec::with_capacity(5_000);

    while grid.contains(position + direction) {
        if grid[position + direction] == b'#' {
            direction = direction.clockwise();
        }

        let next = position + direction;

        if grid[next] == b'.' {
            path.push((position, direction));
            grid[next] = b'^';
        }
        position = next;
    }

    let mut seen: HashSet<(Pos, Pos)> = HashSet::new();
    path.iter()
        .filter(|(position, dir)| {
            seen.clear();
            is_cycle(&grid, &mut seen, *position, *dir)
        })
        .count()
}

fn is_cycle(
    grid: &Grid<u8>,
    seen: &mut HashSet<(Pos, Pos)>,
    mut position: Pos,
    mut dir: Pos,
) -> bool {
    // Instead of cloning the grid, store the obstacle position.
    let obstacle = position + dir;
    if !grid.contains(obstacle) {
        return false;
    }
    // The obstacle is forced to be '#' in the simulation.
    while grid.contains(position) {
        if !seen.insert((position, dir)) {
            return true;
        }
        let next = position + dir;
        if !grid.contains(next) {
            break;
        }
        // If next equals the obstacle, treat as if it were '#'.
        let cell = if next == obstacle { b'#' } else { grid[next] };
        if cell == b'#' {
            dir = dir.clockwise();
            continue;
        }
        position = next;
    }
    false
}
