pub fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    // Parse each line into (test_value, numbers)
    input
        .lines()
        .map(|line| {
            // Split at the colon
            let (target, rest) = line.split_once(':').unwrap();
            let target = target.trim().parse().unwrap();
            let numbers = rest
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (target, numbers)
        })
        .collect()
}

pub fn part1(equations: &[(usize, Vec<usize>)]) -> usize {
    let mut total = 0;

    // Iterate over each equation
    for (target, numbers) in equations {
        // For each equation, try every combination of operators.
        // There are 2^(numbers.len()-1) combinations. Use a bitmask loop.
        let slots = numbers.len() - 1;
        let mut valid = false;
        for mask in 0..(1 << slots) {
            // Evaluate left-to-right. Use first number as starting accumulator.
            let mut result = numbers[0];
            // Apply each operator in order: 0 -> +, 1 -> *
            for i in 0..slots {
                if (mask >> i) & 1 == 1 {
                    result *= numbers[i + 1];
                } else {
                    result += numbers[i + 1];
                }
            }
            if result == *target {
                valid = true;
                break;
            }
        }
        if valid {
            total += target;
        }
    }
    total
}

pub fn part2(input: &[(usize, Vec<usize>)]) -> usize {
    let mut total = 0;
    for (target, numbers) in input {
        let slots = numbers.len() - 1;
        let combo_count = 3_usize.pow(slots as u32);
        let mut valid = false;

        for mask in 0..combo_count {
            let mut result = numbers[0];
            let mut current = mask;
            let mut valid_combo = true;

            for i in 0..slots {
                let op = current % 3;
                current /= 3;
                let right = numbers[i + 1];

                result = match op {
                    0 => result + right,
                    1 => result * right,
                    2 => {
                        let digits = if right == 0 {
                            1
                        } else {
                            (right as f64).log10().floor() as usize + 1
                        };
                        result * 10_usize.pow(digits as u32) + right
                    }
                    _ => {
                        valid_combo = false;
                        break;
                    }
                }
            }

            if valid_combo && result == *target {
                valid = true;
                break;
            }
        }
        if valid {
            total += target;
        }
    }
    total
}
