pub struct Backpack {
    foods: Vec<u32>,
}

pub fn parse(input: &str) -> Vec<Backpack> {
    input
        .split("\n\n")
        .map(|e| Backpack {
            foods: e
                .lines()
                .map(|l| l.trim().parse::<u32>().unwrap())
                .collect(),
        })
        .collect()
}

pub fn part1(input: &[Backpack]) -> String {
    let val: Vec<u32> = input
        .iter()
        .map(|b| b.foods.iter().fold(0, |calories, &food| calories + food))
        .collect();
    return val.iter().max().unwrap().to_string();
}

pub fn part2(input: &[Backpack]) -> String {
    let mut val: Vec<u32> = input
        .iter()
        .map(|b| b.foods.iter().fold(0, |calories, &food| calories + food))
        .collect();
    val.sort();
    return val.iter().rev().take(3).sum::<u32>().to_string();
}
