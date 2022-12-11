use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::{character::complete::digit1, combinator::map_res, sequence::separated_pair, IResult};

fn usize_parser(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

type Pair = (usize, usize);

struct ElfPair {
    p1: Pair,
    p2: Pair,
}

impl ElfPair {
    fn full_overlap(&self) -> bool {
        if self.p1.0 <= self.p2.0 && self.p1.1 >= self.p2.1 {
            return true;
        }

        if self.p2.0 <= self.p1.0 && self.p2.1 >= self.p1.1 {
            return true;
        }

        false
    }

    fn partial_overlap(&self) -> bool {
        // 1-2 1-3
        // 1-2 2-3
        // 2-3 1-2
        if self.p2.0 >= self.p1.0 && self.p2.1 <= self.p1.1 {
            return true;
        }

        if self.p1.0 >= self.p2.0 && self.p1.1 <= self.p2.1 {
            return true;
        }

        if self.p2.0 <= self.p1.1 && self.p2.0 >= self.p1.0 {
            return true;
        }

        if self.p1.0 <= self.p2.1 && self.p1.0 >= self.p2.0 {
            return true;
        }

        false
    }
}

fn parse_pair(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, result) = separated_pair(digit1, char('-'), digit1)(input)?;
    let (_, val1) = usize_parser(result.0)?;
    let (_, val2) = usize_parser(result.1)?;

    IResult::Ok((input, (val1, val2)))
}

fn parse_elf_pair(input: &str) -> IResult<&str, ElfPair> {
    let (input, (p1, p2)) = separated_pair(parse_pair, char(','), parse_pair)(input)?;

    let elf_pair = ElfPair { p1, p2 };

    IResult::Ok((input, elf_pair))
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<ElfPair> {
    let result = separated_list1(char('\n'), parse_elf_pair)(input);

    result.ok().expect("Bad parse").1
}

#[aoc(day4, part1)]
fn part1(input: &Vec<ElfPair>) -> usize {
    input
        .iter()
        .filter(|elf_pair| elf_pair.full_overlap())
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Vec<ElfPair>) -> usize {
    input
        .iter()
        .filter(|elf_pair| elf_pair.partial_overlap())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn part2_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part2(&input), 4);
    }
}
