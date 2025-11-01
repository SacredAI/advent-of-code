type Input = Vec<Robot>;

#[derive(Debug, Clone)]
pub struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            // Parse lines like "p=0,4 v=3,-3"
            let parts: Vec<&str> = line.split_whitespace().collect();
            let pos = parts[0].trim_start_matches("p=");
            let vel = parts[1].trim_start_matches("v=");

            let pos_coords: Vec<i32> = pos.split(',').map(|s| s.parse().unwrap()).collect();
            let vel_coords: Vec<i32> = vel.split(',').map(|s| s.parse().unwrap()).collect();

            Robot {
                x: pos_coords[0],
                y: pos_coords[1],
                vx: vel_coords[0],
                vy: vel_coords[1],
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    const SECONDS: i32 = 100;

    // Simulate each robot's position after 100 seconds
    let final_positions: Vec<(i32, i32)> = input
        .iter()
        .map(|robot| {
            // Calculate final position with wrapping
            let mut final_x = (robot.x + robot.vx * SECONDS) % WIDTH;
            let mut final_y = (robot.y + robot.vy * SECONDS) % HEIGHT;

            // Handle negative modulo
            if final_x < 0 {
                final_x += WIDTH;
            }
            if final_y < 0 {
                final_y += HEIGHT;
            }

            (final_x, final_y)
        })
        .collect();

    // Count robots in each quadrant
    let mid_x = WIDTH / 2;
    let mid_y = HEIGHT / 2;

    let mut quadrants = [0u32; 4];

    for (x, y) in final_positions {
        // Skip robots exactly in the middle
        if x == mid_x || y == mid_y {
            continue;
        }

        let quadrant = match (x < mid_x, y < mid_y) {
            (true, true) => 0,   // Top-left
            (false, true) => 1,  // Top-right
            (true, false) => 2,  // Bottom-left
            (false, false) => 3, // Bottom-right
        };

        quadrants[quadrant] += 1;
    }

    // Calculate safety factor
    quadrants.iter().product()
}

pub fn part2(input: &Input) -> u32 {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

    // The robots will cycle after lcm(WIDTH, HEIGHT) steps
    // Since both are prime, this is WIDTH * HEIGHT
    let max_iterations = WIDTH * HEIGHT;

    // Look for the iteration where robots form a pattern
    // A Christmas tree would likely have all robots at unique positions
    // or show strong clustering
    for seconds in 1..max_iterations {
        let positions: std::collections::HashSet<(i32, i32)> = input
            .iter()
            .map(|robot| {
                let mut x = (robot.x + robot.vx * seconds) % WIDTH;
                let mut y = (robot.y + robot.vy * seconds) % HEIGHT;

                if x < 0 {
                    x += WIDTH;
                }
                if y < 0 {
                    y += HEIGHT;
                }

                (x, y)
            })
            .collect();

        // If all robots are at unique positions, this might be the pattern
        if positions.len() == input.len() {
            // Double-check by looking for clustering or other patterns
            // A Christmas tree would have robots forming connected regions

            // Create a grid to visualize
            let mut has_tree_pattern = false;

            // Look for a vertical line of robots (tree trunk or center)
            // Count how many consecutive vertical positions we have
            for check_x in 0..WIDTH {
                let mut consecutive = 0;
                let mut max_consecutive = 0;
                for check_y in 0..HEIGHT {
                    if positions.contains(&(check_x, check_y)) {
                        consecutive += 1;
                        max_consecutive = max_consecutive.max(consecutive);
                    } else {
                        consecutive = 0;
                    }
                }
                // If we have a long vertical line (>= 10), might be the tree
                if max_consecutive >= 10 {
                    has_tree_pattern = true;
                    break;
                }
            }

            if has_tree_pattern {
                return seconds as u32;
            }
        }
    }

    0
}
