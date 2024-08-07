use nom::{
    IResult,
    bytes::complete::{tag, take_until1},
    multi::separated_list1,
};

use crate::ParsedMove;


pub fn parse_move_internal(input: &str) -> IResult<&str, ParsedMove> {
    let (input, left) = parse_left(input)?;
    let (input, _) = tag(";")(input)?;
    let (input, right) = parse_right(input)?;

    Ok((input, ParsedMove {
        from: left as usize,
        to: right.into_iter().map(|n| n as usize).collect()
    }))
}
fn parse_left(input: &str) -> IResult<&str, u32> {
    nom::character::complete::u32(input)
}

fn parse_right(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag("+"), nom::character::complete::u32)(input)

}
