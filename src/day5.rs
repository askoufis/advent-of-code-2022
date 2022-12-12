use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::char;
use nom::character::complete::{anychar, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

use crate::parsers::parse_usize;

#[derive(Debug)]
struct Move {
    num_crates: usize,
    from_stack: usize,
    to_stack: usize,
}

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, result) = delimited(char('['), anychar, char(']'))(input)?;

    IResult::Ok((input, Some(result)))
}

fn parse_crate_space(input: &str) -> IResult<&str, Option<char>> {
    let (input, _) = tag("   ")(input)?;

    IResult::Ok((input, Some(' ')))
}

fn parse_crate_row(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(char(' '), alt((parse_crate, parse_crate_space)))(input)
}

fn parse_crates(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    separated_list1(char('\n'), parse_crate_row)(input)
}

fn parse_move_n(input: &str) -> IResult<&str, usize> {
    delimited(tag("move "), parse_usize, tag(" from "))(input)
}

fn parse_n_to_n(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(parse_usize, tag(" to "), parse_usize)(input)
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, num_crates) = parse_move_n(input)?;
    let (input, (from_stack, to_stack)) = parse_n_to_n(input)?;

    let move_struct = Move {
        num_crates,
        from_stack,
        to_stack,
    };

    IResult::Ok((input, move_struct))
}

#[derive(Clone)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn move_n_crates(&mut self, n: usize, stack_from: usize, stack_to: usize) {
        for _ in 0..n {
            match self.stacks[stack_from - 1].pop() {
                Some(c) => self.stacks[stack_to - 1].push(c),
                None => panic!("No crates left on stack"),
            }
        }
    }

    fn move_n_crates_ordered(&mut self, n: usize, stack_from: usize, stack_to: usize) {
        let mut crates: Vec<char> = vec![];

        for _ in 0..n {
            match self.stacks[stack_from - 1].pop() {
                Some(c) => crates.push(c),
                None => panic!("No crates left on stack"),
            }
        }

        for _ in 0..n {
            match crates.pop() {
                Some(c) => self.stacks[stack_to - 1].push(c),
                None => panic!("No crates left on stack"),
            }
        }
    }
}

fn take_until_double_newline(input: &str) -> IResult<&str, &str> {
    take_until("\n\n")(input)
}

fn take_newline(input: &str) -> IResult<&str, char> {
    char('\n')(input)
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> (Stacks, Vec<Move>) {
    let (input, mut crates) = parse_crates(input).ok().unwrap();
    // all crate vectors should be the same length
    let num_stacks = crates[0].len();
    crates.reverse();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];
    crates.iter().for_each(|crate_vec| {
        crate_vec
            .iter()
            .enumerate()
            .for_each(|(index, ch)| match ch {
                Some(c) => {
                    if c != &' ' {
                        stacks[index].push(*c)
                    }
                }
                None => {}
            })
    });

    // the newline at the end of the last crate row
    let (input, _) = take_newline(input).ok().unwrap();
    // the number line
    let (input, _) = take_until_double_newline(input).ok().unwrap();
    // the empty line
    let (input, _) = take_newline(input).ok().unwrap();
    let (input, _) = take_newline(input).ok().unwrap();

    let (_, moves) = separated_list1(char('\n'), parse_move)(input).ok().unwrap();
    (Stacks { stacks }, moves)
}

#[aoc(day5, part1)]
fn part1(input: &(Stacks, Vec<Move>)) -> String {
    let mut stacks = input.0.clone();
    input
        .1
        .iter()
        .for_each(|m| stacks.move_n_crates(m.num_crates, m.from_stack, m.to_stack));
    stacks.stacks.iter().map(|s| s.last().unwrap()).collect()
}

#[aoc(day5, part2)]
fn part2(input: &(Stacks, Vec<Move>)) -> String {
    let mut stacks = input.0.clone();
    input
        .1
        .iter()
        .for_each(|m| stacks.move_n_crates_ordered(m.num_crates, m.from_stack, m.to_stack));
    stacks.stacks.iter().map(|s| s.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test1() {
        let input = "[A]     [B]     [C]";
        let result: Vec<char> = parse_crate_row(input)
            .ok()
            .expect("bad parse")
            .1
            .into_iter()
            .filter_map(|x| x)
            .collect();
        assert_eq!(result, vec!['A', ' ', 'B', ' ', 'C']);
    }

    #[test]
    fn parser_test2() {
        let input = "    [A]     [B]    ";
        let result: Vec<char> = parse_crate_row(input)
            .ok()
            .expect("bad parse")
            .1
            .into_iter()
            .filter_map(|x| x)
            .collect();
        assert_eq!(result, vec![' ', 'A', ' ', 'B', ' ']);
    }

    const INPUT_STR: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part1(&input), "CMZ");
    }

    #[test]
    fn part2_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part2(&input), "MCD");
    }
}
