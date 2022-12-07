type Calories = usize;

type ElfFoodList = Vec<Vec<Calories>>;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> ElfFoodList {
    input
        .split("\n\n")
        .map(|items| {
            items
                .split("\n")
                .map(|item| item.parse().expect("Failed to parse string to usize"))
                .collect()
        })
        .collect()
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
mod tests {}
