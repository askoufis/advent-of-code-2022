use core::num;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, multispace1, space1};
use nom::combinator::{map_res, recognize};
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;

fn usize_parser(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn monkey_id(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, id) = usize_parser(input)?;
    let (input, _) = char(':')(input)?;

    IResult::Ok((input, id))
}

fn starting_items(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Starting items: ")(input)?;
    separated_list1(tag(", "), map_res(digit1, str::parse::<usize>))(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, symbol) = alt((char('*'), char('+')))(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = alt((tag("old"), digit1))(input)?;

    let operation = match value {
        "old" => Operation::OldSquared,
        v => {
            let val = str::parse(v).expect("Bad value");
            Operation::from((symbol, Some(val)))
        }
    };

    IResult::Ok((input, operation))
}

fn test_divisible(input: &str) -> IResult<&str, usize> {
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Test: divisible by ")(input)?;
    map_res(recognize(digit1), str::parse)(input)
}

fn if_true(input: &str) -> IResult<&str, usize> {
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("If true: throw to monkey ")(input)?;
    map_res(recognize(digit1), str::parse)(input)
}

fn if_false(input: &str) -> IResult<&str, usize> {
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("If false: throw to monkey ")(input)?;
    map_res(recognize(digit1), str::parse)(input)
}

fn test_whole(input: &str) -> IResult<&str, Test> {
    let (input, divisible_by) = terminated(test_divisible, char('\n'))(input)?;
    let (input, true_to) = terminated(if_true, char('\n'))(input)?;
    // Don't consume newline here as it's the end of the item
    let (input, false_to) = if_false(input)?;

    let test = Test {
        divisible_by,
        true_to,
        false_to,
    };

    IResult::Ok((input, test))
}

fn monkey_whole(input: &str) -> IResult<&str, Monkey> {
    let (input, id) = terminated(monkey_id, char('\n'))(input)?;
    let (input, items) = starting_items(input)?;
    let (input, _) = char('\n')(input)?;
    let (input, operation) = terminated(operation, char('\n'))(input)?;
    let (input, test) = test_whole(input)?;

    let monkey = Monkey {
        id,
        items,
        operation,
        test,
        inspect_count: 0,
    };

    IResult::Ok((input, monkey))
}

#[derive(Debug, Clone)]
enum Operation {
    OldAdd(usize),
    OldMultiply(usize),
    OldSquared,
}

impl Operation {
    fn apply(&self, old: usize) -> usize {
        match self {
            Operation::OldAdd(val) => old + val,
            Operation::OldMultiply(val) => old * val,
            Operation::OldSquared => old * old,
        }
    }
}

impl From<(char, Option<usize>)> for Operation {
    fn from((c, val): (char, Option<usize>)) -> Self {
        match val {
            Some(v) => match c {
                '*' => Self::OldMultiply(v),
                '+' => Self::OldAdd(v),
                _ => panic!("Bad operation symbol"),
            },
            None => Self::OldSquared,
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    divisible_by: usize,
    true_to: usize,
    false_to: usize,
}

impl Test {
    fn apply(&self, worry_level: usize) -> usize {
        let result = worry_level % self.divisible_by;

        match result {
            0 => self.true_to,
            _ => self.false_to,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    inspect_count: usize,
}

struct Throw {
    worry_level: usize,
    to_monkey_id: usize,
}

impl Monkey {
    fn inspect(&mut self, lcm: usize, disable_worry_division: bool) -> Throw {
        let old_worry_level = self.items.get(0).expect("No element and index 0");
        let mut worry_level = self.operation.apply(*old_worry_level);
        if !disable_worry_division {
            worry_level /= 3;
        }
        if worry_level >= lcm {
            worry_level %= lcm;
        }
        let to_monkey_id = self.test.apply(worry_level);

        // Pop from front of items
        let new_items = &self.items[1..];
        self.items = new_items.to_vec();

        self.inspect_count += 1;

        Throw {
            worry_level,
            to_monkey_id,
        }
    }

    fn accept_throw(&mut self, throw: &Throw) {
        self.items.push(throw.worry_level)
    }
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<Monkey> {
    let result = separated_list1(tag("\n\n"), monkey_whole)(input);
    result.ok().expect("Bad monkeys").1
}

#[aoc(day11, part1)]
fn part1(input: &Vec<Monkey>) -> usize {
    let rounds = 20;
    let mut monkeys = input.clone();
    let num_monkeys = monkeys.len();
    let lcm: usize = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product();
    let disable_worry_division = false;

    for _ in 0..rounds {
        for m in 0..num_monkeys {
            let mut throws = vec![];
            let monkey = &mut monkeys[m];

            while monkey.items.len() > 0 {
                let throw = monkey.inspect(lcm, disable_worry_division);
                throws.push(throw);
            }

            for throw in &mut throws {
                let monkey_id = throw.to_monkey_id;
                monkeys[monkey_id].accept_throw(&throw)
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    monkeys.iter().take(2).map(|m| m.inspect_count).product()
}

#[aoc(day11, part2)]
fn part2(input: &Vec<Monkey>) -> usize {
    let rounds = 10000;
    let mut monkeys = input.clone();
    let lcm: usize = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product();
    let num_monkeys = monkeys.len();
    let disable_worry_division = true;

    for _ in 0..rounds {
        for m in 0..num_monkeys {
            let mut throws = vec![];
            let monkey = &mut monkeys[m];

            while monkey.items.len() > 0 {
                let throw = monkey.inspect(lcm, disable_worry_division);
                throws.push(throw);
            }

            for throw in &mut throws {
                let monkey_id = throw.to_monkey_id;
                monkeys[monkey_id].accept_throw(&throw)
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    monkeys.iter().take(2).map(|m| m.inspect_count).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part1(&input), 10605);
    }

    #[test]
    fn part2_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part2(&input), 2713310158);
    }
}
