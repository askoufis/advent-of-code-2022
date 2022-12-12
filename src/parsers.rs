use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}
