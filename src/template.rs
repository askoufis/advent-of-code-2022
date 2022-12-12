#[aoc_generator(day11)]
fn input_generator(input: &str) -> usize {
    1
}

#[aoc(day11, part1)]
fn part1(input: &usize) -> usize {
    *input
}

#[aoc(day11, part2)]
fn part2(input: &usize) -> usize {
    *input
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = r"";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part1(&input), 1);
    }

    #[test]
    fn part2_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part2(&input), 1);
    }
}
