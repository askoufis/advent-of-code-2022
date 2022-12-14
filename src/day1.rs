use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::separated_list1;

use crate::parsers::parse_usize;

type Calories = usize;

type ElfFoodList = Vec<Vec<Calories>>;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> ElfFoodList {
    separated_list1(tag("\n\n"), separated_list1(char('\n'), parse_usize))(input)
        .ok()
        .unwrap()
        .1
}

#[aoc(day1, part1)]
fn part1(input: &ElfFoodList) -> usize {
    input
        .iter()
        .map(|items| items.iter().sum())
        .max()
        .expect("No max")
}

#[aoc(day1, part2)]
fn part2(input: &ElfFoodList) -> usize {
    let mut calories_per_elf: Vec<usize> = input.iter().map(|items| items.iter().sum()).collect();
    calories_per_elf.sort();
    calories_per_elf.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn part2_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part2(&input), 45000);
    }
}
