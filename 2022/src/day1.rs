
pub struct Backpack {
    foods: Vec<u32>
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Backpack> {
    input.split("\n\n").map(|e| {
        Backpack {
         foods: e.lines().map(|l| l.trim().parse::<u32>().unwrap()).collect()
        }
    }).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Backpack]) -> String {
    let val: Vec<u32> = input
    .iter()
    .map(|b| b.foods.iter().fold(0, |calories, &food| {
        calories + food
    }))
    .collect();
    return val.iter().max().unwrap().to_string();
}

#[aoc(day1, part2)]
pub fn part2(input: &[Backpack]) -> String {
    let mut val: Vec<u32> = input
    .iter()
    .map(|b| b.foods.iter().fold(0, |calories, &food| {
        calories + food
    }))
    .collect();
    val.sort();
    return val.iter().rev().take(3).sum::<u32>().to_string();
}

#[cfg(test)]
mod tests{
    use crate::day1::input_generator;

    #[test]
    fn p1(){
        let input = input_generator("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000");
        let result = crate::day1::part1(&input);
        assert_eq!(result, "24000");
    }

    #[test]
    fn p2(){
        let input = input_generator("1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000");
        let result = crate::day1::part2(&input);
        assert_eq!(result, "45000");
    }
}
