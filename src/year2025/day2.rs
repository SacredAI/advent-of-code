type Input = Vec<(u64, u64)>;

pub fn parse(input: &str) -> Input {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect()
}

/// Generate all invalid IDs (pattern repeated exactly twice) that fall within a given range
fn find_invalid_ids_in_range(start: u64, end: u64) -> Vec<u64> {
    let mut invalid_ids = Vec::new();

    let start_digits = if start == 0 {
        1
    } else {
        (start as f64).log10().floor() as u32 + 1
    };
    let end_digits = if end == 0 {
        1
    } else {
        (end as f64).log10().floor() as u32 + 1
    };

    // We need even-length numbers for exactly 2 repetitions
    let min_pattern_len = (start_digits + 1) / 2;
    let max_pattern_len = end_digits / 2 + 1;

    for pattern_len in min_pattern_len..=max_pattern_len {
        let pattern_start = if pattern_len == 1 {
            1
        } else {
            10_u64.pow(pattern_len - 1)
        };
        let pattern_end = 10_u64.pow(pattern_len);

        for pattern in pattern_start..pattern_end {
            // Create the invalid ID by repeating the pattern twice
            let invalid_id = pattern * 10_u64.pow(pattern_len) + pattern;

            if invalid_id >= start && invalid_id <= end {
                invalid_ids.push(invalid_id);
            }
        }
    }

    invalid_ids
}

/// Generate all invalid IDs (pattern repeated at least twice) that fall within a given range
fn find_invalid_ids_in_range_part2(start: u64, end: u64) -> Vec<u64> {
    use std::collections::HashSet;
    let mut invalid_ids = HashSet::new();

    let start_digits = if start == 0 {
        1
    } else {
        (start as f64).log10().floor() as u32 + 1
    };
    let end_digits = if end == 0 {
        1
    } else {
        (end as f64).log10().floor() as u32 + 1
    };

    // For each total number of digits we might encounter
    for total_digits in start_digits..=end_digits {
        // Pattern length must divide total_digits, and repetitions >= 2
        for pattern_len in 1..=total_digits / 2 {
            if total_digits % pattern_len != 0 {
                continue;
            }
            let repetitions = total_digits / pattern_len;
            if repetitions < 2 {
                continue;
            }

            // Generate all patterns of this length (no leading zeros)
            let pattern_start = if pattern_len == 1 {
                1
            } else {
                10_u64.pow(pattern_len - 1)
            };
            let pattern_end = 10_u64.pow(pattern_len);

            for pattern in pattern_start..pattern_end {
                // Create the invalid ID by repeating the pattern
                let mut invalid_id: u64 = 0;
                let multiplier = 10_u64.pow(pattern_len);
                for _ in 0..repetitions {
                    invalid_id = invalid_id * multiplier + pattern;
                }

                if invalid_id >= start && invalid_id <= end {
                    invalid_ids.insert(invalid_id);
                }
            }
        }
    }

    invalid_ids.into_iter().collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut sum = 0;

    for &(start, end) in input {
        let invalid_ids = find_invalid_ids_in_range(start, end);
        for id in invalid_ids {
            sum += id;
        }
    }

    sum
}

pub fn part2(input: &Input) -> u64 {
    let mut sum = 0;

    for &(start, end) in input {
        let invalid_ids = find_invalid_ids_in_range_part2(start, end);
        for id in invalid_ids {
            sum += id;
        }
    }

    sum
}
