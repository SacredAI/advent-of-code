use aoc::util::ansi::*;
use aoc::util::parse::*;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use clap::Parser;
use std::iter::empty;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Specifies the day. Defaults to last implemented.
    #[clap(short, long)]
    day: u32,

    #[clap(short, long)]
    year: u32,
}

struct Solution {
    year: u32,
    day: u32,
    path: PathBuf,
    wrapper: fn(String) -> (String, String),
}

fn main() {
    let cli = Cli::parse();

    let solutions = empty()
        .chain(year2022())
        .chain(year2023())
        .chain(year2024())
        .filter(|soln| soln.day == cli.day && soln.year == cli.year)
        .collect::<Vec<_>>();

    // Pretty print output for each solution.
    let mut duration = Duration::ZERO;

    for Solution {
        year,
        day,
        path,
        wrapper,
    } in &solutions
    {
        if let Ok(data) = read_to_string(path) {
            let instant = Instant::now();
            let (part1, part2) = wrapper(data);
            duration += instant.elapsed();

            println!("{BOLD}{YELLOW}{year} Day {day:02}{RESET}");
            println!("    Part 1: {part1}");
            println!("    Part 2: {part2}");
        } else {
            eprintln!("{BOLD}{RED}{year} Day {day:02}{RESET}");
            eprintln!("    Missing input!");
            eprintln!(
                "    Place input file in {BOLD}{WHITE}{}{RESET}",
                path.display()
            );
        }
    }
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$({
            let year: u32 = stringify!($year).unsigned();
            let year_str = year.to_string();
            let day = stringify!($day);
            let path = Path::new("./input").join(year_str).join(day).with_extension("txt");

            let wrapper = |data: String| {
                use aoc::$year::$day::*;

                let input = parse(&data);
                let part1 = part1(&input);
                let part2 = part2(&input);

                (part1.to_string(), part2.to_string())
            };
            Solution { year: year, day: day.unsigned(), path, wrapper}
        },)*]
    }
    }
}

run!(year2022 day1, day2, day3, day4, day5);

run!(year2023 day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14);

run!(year2024 day1, day2, day3, day4, day5);
