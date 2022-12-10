use std::convert::From;

#[derive(Clone, Copy)]
enum Left {
    A,
    B,
    C,
}

impl From<char> for Left {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            _ => panic!("Could not convert char '{}'", c),
        }
    }
}

#[derive(Clone, Copy)]
enum Right {
    X,
    Y,
    Z,
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn vs(&self, opponent: &Self) -> usize {
        match (self, opponent) {
            (Move::Rock, Move::Rock) => 4,
            (Move::Rock, Move::Paper) => 1,
            (Move::Rock, Move::Scissors) => 7,
            (Move::Paper, Move::Rock) => 8,
            (Move::Paper, Move::Paper) => 5,
            (Move::Paper, Move::Scissors) => 2,
            (Move::Scissors, Move::Rock) => 3,
            (Move::Scissors, Move::Paper) => 9,
            (Move::Scissors, Move::Scissors) => 6,
        }
    }
}

impl From<Left> for Move {
    fn from(l: Left) -> Self {
        match l {
            Left::A => Self::Rock,
            Left::B => Self::Paper,
            Left::C => Self::Scissors,
        }
    }
}

impl From<Right> for Move {
    fn from(l: Right) -> Self {
        match l {
            Right::X => Self::Rock,
            Right::Y => Self::Paper,
            Right::Z => Self::Scissors,
        }
    }
}

impl From<char> for Right {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::X,
            'Y' => Self::Y,
            'Z' => Self::Z,
            _ => panic!("Could not convert char '{}'", c),
        }
    }
}

enum RoundResult {
    Win,
    Draw,
    Loss,
}

impl RoundResult {
    fn your_move_from_opponent(&self, opponent: &Move) -> Move {
        match (self, opponent) {
            (RoundResult::Win, Move::Rock) => Move::Paper,
            (RoundResult::Draw, Move::Rock) => Move::Rock,
            (RoundResult::Loss, Move::Rock) => Move::Scissors,
            (RoundResult::Win, Move::Paper) => Move::Scissors,
            (RoundResult::Draw, Move::Paper) => Move::Paper,
            (RoundResult::Loss, Move::Paper) => Move::Rock,
            (RoundResult::Win, Move::Scissors) => Move::Rock,
            (RoundResult::Draw, Move::Scissors) => Move::Scissors,
            (RoundResult::Loss, Move::Scissors) => Move::Paper,
        }
    }
}

impl From<Right> for RoundResult {
    fn from(l: Right) -> Self {
        match l {
            Right::X => Self::Loss,
            Right::Y => Self::Draw,
            Right::Z => Self::Win,
        }
    }
}

struct StrategyGuide {
    rounds: Vec<(Left, Right)>,
}

fn score(round: &(Left, Right)) -> usize {
    let opponent = Move::from(round.0);
    let you = Move::from(round.1);

    you.vs(&opponent)
}

fn score2(round: &(Left, Right)) -> usize {
    let opponent = Move::from(round.0);
    let result = RoundResult::from(round.1);

    let you = result.your_move_from_opponent(&opponent);
    you.vs(&opponent)
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> StrategyGuide {
    let rounds = input
        .split("\n")
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            println!("line: {}, chars: {:#?}", &line, &chars);
            let left = chars[0];
            let right = chars[2];
            (Left::from(left), Right::from(right))
        })
        .collect();

    StrategyGuide { rounds }
}

#[aoc(day2, part1)]
fn part1(input: &StrategyGuide) -> usize {
    input.rounds.iter().map(score).sum()
}

#[aoc(day2, part2)]
fn part2(input: &StrategyGuide) -> usize {
    input.rounds.iter().map(score2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = r"A Y
B X
C Z";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn part2_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part2(&input), 12);
    }
}
