use std::collections::HashSet;

use nom::{character::complete::anychar, multi::many1, IResult};

fn parse_chars(input: &str) -> IResult<&str, Vec<char>> {
    many1(anychar)(input)
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<char> {
    parse_chars(input).ok().unwrap().1
}

#[aoc(day6, part1)]
fn part1(input: &Vec<char>) -> usize {
    let marker_length = 4;
    input
        .windows(marker_length)
        .take_while(|window| {
            let mut set: HashSet<char> = HashSet::new();
            window.iter().for_each(|c| {
                set.insert(*c);
            });
            set.len() != marker_length
        })
        .count()
        + marker_length
}

#[aoc(day6, part2)]
fn part2(input: &Vec<char>) -> usize {
    let marker_length = 14;
    input
        .windows(marker_length)
        .take_while(|window| {
            let mut set: HashSet<char> = HashSet::new();
            window.iter().for_each(|c| {
                set.insert(*c);
            });
            set.len() != marker_length
        })
        .count()
        + marker_length
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn part2_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part2(&input), 19);
    }
}
