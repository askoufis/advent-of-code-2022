use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, terminated},
    IResult,
};

use crate::parsers::parse_usize;

#[derive(Debug, Clone)]
enum Value {
    Int(usize),
    List(Vec<Value>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Int(_), Value::List(_)) => false,
            (Value::List(_), Value::Int(_)) => false,
            (Value::List(a), Value::List(b)) => {
                if a.len() != b.len() {
                    return false;
                } else {
                    return a.iter().zip(b).all(|(first, second)| first == second);
                }
            }
        }
    }
}

impl Eq for Value {}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(inner) => write!(f, "{inner}"),
            Value::List(inner) => write!(f, "{:?}", inner),
        }
    }
}

impl Value {
    fn compare(&self, other: &Value) -> Option<bool> {
        match (self, other) {
            (Value::Int(first), Value::Int(second)) => {
                if first < second {
                    return Some(true);
                }
                if first > second {
                    return Some(false);
                }
                None
            }
            (Value::Int(first), Value::List(_)) => {
                let first_list = Self::List(vec![Self::Int(*first)]);
                first_list.compare(other)
            }
            (Value::List(_), Value::Int(second)) => {
                let second_list = Self::List(vec![Self::Int(*second)]);
                self.compare(&second_list)
            }
            (Value::List(first), Value::List(second)) => compare_packets(first, second),
        }
    }
}

fn compare_packets(first: &Packet, second: &Packet) -> Option<bool> {
    let mut index = 0;
    while first.len() > index && second.len() > index {
        let d1 = &first[index];
        let d2 = &second[index];

        let result = d1.compare(d2);
        if let Some(b) = result {
            return Some(b);
        }
        index += 1;
    }

    // at this point, comparing items has stopped, so we compare lengths
    if first.len() < second.len() {
        return Some(true);
    }

    if first.len() > second.len() {
        return Some(false);
    }

    // The lists are of even length
    None
}

type Packet = Vec<Value>;

#[derive(Debug, Clone)]
struct PacketPair {
    first: Packet,
    second: Packet,
}

impl PacketPair {
    fn compare(&self) -> bool {
        let result = compare_packets(&self.first, &self.second);

        if let Some(true) = result {
            return true;
        }

        return false;
    }
}

fn parse_item(input: &str) -> IResult<&str, Value> {
    let (input, result) = parse_usize(input)?;

    IResult::Ok((input, Value::Int(result)))
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    let item = alt((parse_item, parse_value));
    let inner = separated_list0(char(','), item);
    let (input, result) = delimited(char('['), inner, char(']'))(input)?;

    IResult::Ok((input, Value::List(result)))
}

fn parse_packet_pair(input: &str) -> IResult<&str, PacketPair> {
    let (input, first) = terminated(parse_value, char('\n'))(input)?;
    let (input, second) = parse_value(input)?;

    match (first, second) {
        (Value::Int(_), Value::Int(_)) => unreachable!("Bad"),
        (Value::Int(_), Value::List(_)) => unreachable!("Bad"),
        (Value::List(_), Value::Int(_)) => unreachable!("Bad"),
        (Value::List(first_packet), Value::List(second_packet)) => IResult::Ok((
            input,
            PacketPair {
                first: first_packet,
                second: second_packet,
            },
        )),
    }
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Vec<PacketPair> {
    separated_list1(tag("\n\n"), parse_packet_pair)(input)
        .ok()
        .unwrap()
        .1
}

#[aoc(day13, part1)]
fn part1(input: &Vec<PacketPair>) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(index, packet_pair)| (index + 1, packet_pair.compare()))
        .filter_map(|(index, result)| result.then(|| index))
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &Vec<PacketPair>) -> usize {
    let mut flat_input: Vec<Packet> = vec![];
    for pair in input {
        flat_input.push(pair.first.clone());
        flat_input.push(pair.second.clone());
    }

    let divider_packet1: Packet = vec![Value::List(vec![Value::Int(2)])];
    let divider_packet2: Packet = vec![Value::List(vec![Value::Int(6)])];
    flat_input.push(divider_packet1.clone());
    flat_input.push(divider_packet2.clone());

    flat_input.sort_by(|a, b| {
        let result = compare_packets(a, b);
        match result {
            Some(b) => {
                if b {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            }
            None => std::cmp::Ordering::Equal,
        }
    });

    let index_1 = flat_input
        .iter()
        .position(|packet| *packet == divider_packet1)
        .unwrap();
    let index_2 = flat_input
        .iter()
        .position(|packet| *packet == divider_packet2)
        .unwrap();

    (index_1 + 1) * (index_2 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn part2_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part2(&input), 140);
    }
}
