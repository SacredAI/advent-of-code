macro_rules! library {
    ($year:tt $($day:tt),*) => {
        pub mod $year {$(pub mod $day;)*}
    }
}

library!(util ansi, parse, integer, position, grid);

library!(year2024 day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14);

library!(year2023 day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14);

library!(year2022 day1, day2, day3, day4, day5);
library!(year2025 day1);
