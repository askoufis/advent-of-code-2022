use std::collections::HashSet;

type Rucksack = Vec<char>;

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Rucksack> {
    input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

fn find_duplicate(rucksack: &Rucksack) -> char {
    let length = rucksack.len();
    let compartment_size = length / 2;
    let (first, second) = rucksack.split_at(compartment_size);
    let first_set = items_to_set(first);
    let second_set = items_to_set(second);

    *first_set
        .intersection(&second_set)
        .take(1)
        .next()
        .expect("Didn't get anything")
}

fn find_badge(rucksacks: &[Rucksack]) -> char {
    let first = &rucksacks[0];
    let second = &rucksacks[1];
    let third = &rucksacks[2];

    let first_set = items_to_set(first);
    let second_set = items_to_set(second);
    let third_set = items_to_set(third);

    let first_intersection: HashSet<char> = first_set
        .intersection(&second_set)
        .map(|c| c.to_owned())
        .collect();

    *first_intersection
        .intersection(&third_set)
        .take(1)
        .next()
        .expect("Didn't get anything")
}

fn items_to_set(items: &[char]) -> HashSet<char> {
    HashSet::from_iter(items.to_vec())
}

fn score(c: &char) -> usize {
    let char_u8 = *c as u8;
    let score = match char_u8 {
        // Lowercase
        97..=122 => char_u8 - 96,
        // Uppercase
        65..=90 => char_u8 - 64 + 26,
        _ => panic!("bad character"),
    };
    println!("char '{}' = {}", c, score);
    score as usize
}

#[aoc(day3, part1)]
fn part1(input: &Vec<Rucksack>) -> usize {
    let duplicates: Vec<_> = input.iter().map(find_duplicate).collect();
    duplicates.iter().map(score).sum()
}

#[aoc(day3, part2)]
fn part2(input: &Vec<Rucksack>) -> usize {
    let badges: Vec<_> = input.chunks(3).map(find_badge).collect();
    badges.iter().map(score).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STRING: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STRING);
        assert_eq!(part1(&input), 157);
    }

    #[test]
    fn part2_test() {
        let input = input_generator(INPUT_STRING);
        assert_eq!(part2(&input), 70);
    }
}
