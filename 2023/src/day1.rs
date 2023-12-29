#[aoc(day1, part1)]
pub fn part1(input: &str) -> String {
    let val: u32 = input
    .lines()
    .map(|l|
        l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>()
    )
    .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
    .sum();
    val.to_string()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> String {
    let val: u32 = input
    .lines()
    .map(|line| {
        line.to_string()
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
    })
    .map(|l|
        l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>()
    )
    .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
    .sum();
    val.to_string()
}

#[cfg(test)]
mod tests{
    #[test]
    fn p1(){
        let result = crate::day1::part1("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, "142");
    }

    #[test]
    fn p2(){
        let result = crate::day1::part2("two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen");
        assert_eq!(result, "281");
    }
}
