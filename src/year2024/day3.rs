use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, anychar},
    combinator::value,
    multi::{fold_many1, many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn instruction(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("mul")(input)?;
    let (input, pairs) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    return Ok((input, pairs.0 * pairs.1));
}

fn find_next(input: &str) -> IResult<&str, u32> {
    let mut input = input;
    loop {
        let result: IResult<&str, u32> = instruction(input);
        match result {
            Ok(v) => {
                break Ok(v);
            }
            Err(_) => {
                let next: IResult<&str, _> = take(1usize)(input);
                match next {
                    Ok((i, _)) => {
                        input = i;
                    }
                    Err(e) => {
                        break (Err(e));
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Eq)]
enum ShouldProcess {
    Do,
    Dont,
}

#[derive(Debug, Clone)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn new_instruct(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u32 {
    fold_many1(
        find_next,
        || 0,
        |mut acc: u32, item| {
            acc += item;
            acc
        },
    )(input)
    .unwrap()
    .1
}

pub fn part2(input: &str) -> u32 {
    let (_, instructions) =
        many1(many_till(anychar, new_instruct).map(|(_discard, ins)| ins))(input).unwrap();

    let (_, result) = instructions
        .iter()
        .fold((ShouldProcess::Do, 0), |(process, acc), ins| match ins {
            Instruction::Mul(a, b) => {
                if process == ShouldProcess::Do {
                    (process, acc + a * b)
                } else {
                    (process, acc)
                }
            }
            Instruction::Do => (ShouldProcess::Do, acc),
            Instruction::Dont => (ShouldProcess::Dont, acc),
        });

    result
}
