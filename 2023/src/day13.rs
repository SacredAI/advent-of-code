fn find_col(grid: &[&[u8]], limit: usize) -> Option<usize> {
    (0..grid[0].len() - 1).find(|&c| {
        let incorrect = (0..=c.min(grid[0].len() - c - 2))
            .map(|dc| {
                let a = c - dc;
                let b = c + 1 + dc;
                (0..grid.len())
                    .filter(|&r| grid[r][a] != grid[r][b])
                    .count()
            })
            .sum::<usize>();
        incorrect == limit
    })
}

fn find_row(grid: &[&[u8]], limit: usize) -> Option<usize> {
    (0..grid.len() - 1).find(|&r| {
        let incorrect = (0..=r.min(grid.len() - r - 2))
            .map(|dr| {
                let a = r - dr;
                let b = r + 1 + dr;
                (0..grid[0].len())
                    .filter(|&c| grid[a][c] != grid[b][c])
                    .count()
            })
            .sum::<usize>();
        incorrect == limit
    })
}

// #[aoc_generator(day13)]
fn input_generator(input: &str) -> Vec<Vec<&[u8]>> {
    input
        .split("\n\n")
        .map(|s| s.split('\n').map(|l| l.as_bytes()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn solve(grids: &[Vec<&[u8]>], limit: usize) -> usize {
    grids
        .iter()
        .map(|grid| {
            find_row(grid, limit)
                .map(|r| (r + 1) * 100)
                .or_else(|| find_col(grid, limit).map(|c| c + 1))
                .unwrap()
        })
        .sum()
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    let grids = input_generator(input);
    solve(&grids, 0)
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    let grids = input_generator(input);
    solve(&grids, 1)
}
