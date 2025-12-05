use aoc::util::ansi::*;
use aoc::util::parse::*;
use std::fs;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use crate::Commands::Template;
use clap::{Parser, Subcommand};
use std::iter::empty;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Option<Commands>,
    /// Specifies the day. Defaults to last implemented.
    #[clap(short, long)]
    day: Option<u32>,

    #[clap(short, long)]
    year: Option<u32>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Template {
        #[clap(short, long)]
        day: u32,
        #[clap(short, long)]
        year: u32,
    },
}

struct Solution {
    year: u32,
    day: u32,
    path: PathBuf,
    wrapper: fn(String) -> (String, String),
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Some(subcmd) => {
            match subcmd {
                Template { year, day } => {
                    let folder = format!("src/year{}", year);
                    if fs::exists(&folder).is_ok_and(|x| !x) {
                        fs::create_dir_all(&folder).expect("Failed to create year folder");
                    }
                    fs::copy(
                        "src/templates/dayn.rs",
                        format!("src/year{}/day{}.rs", year, day),
                    )
                    .expect("Failed to Copy template file");
                    // New: update run! and library! macros with the new day
                    update_macros(year, day);
                }
            }

            return;
        }
        None => {}
    }

    let solutions = empty()
        .chain(year2022())
        .chain(year2023())
        .chain(year2024())
        .chain(year2025())
        .filter(|soln| {
            soln.day == cli.day.expect("Must provide day to run")
                && soln.year == cli.year.expect("Must provide year to run")
        })
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

// New function that updates run! and library! macro calls in this file.
fn update_macros(year: u32, day: u32) {
    update_run_macro(year, day);
    update_library_macro(year, day);
}

fn update_run_macro(year: u32, day: u32) {
    use std::io::Write;
    let file_path = "src/main.rs";
    let content = fs::read_to_string(file_path).expect("Failed to read main.rs");
    let mut lines: Vec<String> = content.lines().map(String::from).collect();
    let run_marker = format!("run!(year{}", year);
    let day_token = format!("day{}", day);
    let mut updated = false;
    for line in &mut lines {
        if line.contains(&run_marker) && !line.contains(&day_token) {
            if let Some(pos) = line.rfind(')') {
                let sep = if line[..pos].ends_with(' ') { "" } else { ", " };
                *line = format!("{}{}{}{});", &line[..pos], sep, day_token, "");
                updated = true;
            }
        }
    }
    if !updated {
        lines.push(format!("run!(year{} {});", year, day_token));
    }
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Failed to open main.rs for writing");
    writeln!(file, "{}", lines.join("\n")).expect("Failed to write updated run! macro");
}

fn update_library_macro(year: u32, day: u32) {
    use std::io::Write;
    let file_path = "src/lib.rs";
    let content = fs::read_to_string(file_path).expect("Failed to read lib.rs");
    let mut lines: Vec<String> = content.lines().map(String::from).collect();
    let lib_marker = format!("library!(year{}", year);
    let day_token = format!("day{}", day);
    let mut updated = false;
    for line in &mut lines {
        if line.contains(&lib_marker) && !line.contains(&day_token) {
            if let Some(pos) = line.rfind(')') {
                let sep = if line[..pos].ends_with(' ') { "" } else { ", " };
                *line = format!("{}{}{}{});", &line[..pos], sep, day_token, "");
                updated = true;
            }
        }
    }
    if !updated {
        lines.push(format!("library!(year{} {});", year, day_token));
    }
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Failed to open lib.rs for writing");
    writeln!(file, "{}", lines.join("\n")).expect("Failed to write updated library! macro");
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

run!(year2024 day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14);
run!(year2025 day1, day2);
