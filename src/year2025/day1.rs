type Input = Vec<i32>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|step| {
            let (dir, numStr) = step.split_at(1);
            let mut num = numStr.parse::<i32>().unwrap();
            if dir == "L" {
                num *= -1;
            }
            num
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    let mut val = 50;
    let mut count = 0;

    for num in input.iter() {
        val += num;
        val = val.rem_euclid(100);
        if val == 0 {
            count += 1;
        }
    }
    count
}

pub fn part2(input: &Input) -> u32 {
    let mut val: i32 = 50;
    let mut count: i32 = 0;

    for num in input.iter() {
        if *num > 0 {
            // Moving right: count multiples of 100 in (val, val + num]
            count += (val + num).div_euclid(100) - val.div_euclid(100);
        } else if *num < 0 {
            // Moving left: count multiples of 100 in [val + num, val)
            count += (val - 1).div_euclid(100) - (val + num - 1).div_euclid(100);
        }
        val = (val + num).rem_euclid(100);
    }
    count as u32
}
