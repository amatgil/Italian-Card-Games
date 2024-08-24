use nom::{
    IResult,
    bytes::complete::tag,
    multi::separated_list0,
    branch::alt,
};

enum ParsedMove {
    RevealNextOfStack,
    MoveFromStackToPile(usize),
    MoveFromPileToPile {
        from: usize,
        to: usize,
        amount: usize
    },
    MoveFromPileToAce { // Lowest card of pile only
        pile: usize,
        ace: usize
    },
    MoveFromAceToPile { // Top card of ace only
        ace: usize
        pile: usize,
    },
    Undo
}

// Syntax:
//  - Reveal next card in stack: `next`
//  - Move top card in stack to pile X: `s;X`
//  - Move N cards from pile X to Y: `mX;Y;N`
//  - Move lowest card from pile X to ace stack Y: `mX;aY`
//  - Move top card from ace stack Y to pile X: `maY;X`
//  - Undo: `u` or `undo`

pub fn parse_move(input: &str) -> IResult<&str, ParsedMove> {
    alt((
            parse_stack_revealing,
            parse_move_stack_to_pile,
            parse_move_pile_to_pile,
            parse_move_pile_to_aces,
            parse_move_aces_to_pile,
            parse_undo,
            ))(input)
}

pub fn parse_stack_revealing(input: &str) -> IResult<&str, ParsedMove> {
    let (input, _) = alt((tag("u"), tag("undo")))(input)?;
    Ok((input, ParsedMove::RevealNextOfStack))
}
pub fn parse_move_stack_to_pile(input: &str) -> IResult<&str, ParsedMove> {
    let (input, _) = tag("s;")?;
    let (input, n) = nom::character::complete::u32(input)?;
    Ok((input, ParsedMove::MoveFromStackToPile(n as usize)))

}

pub fn parse_move_pile_to_pile(input: &str) -> IResult<&str, ParsedMove> {
    todo!()
}
pub fn parse_move_pile_to_aces(input: &str) -> IResult<&str, ParsedMove> {
    todo!()
}
pub fn parse_move_aces_to_pile(input: &str) -> IResult<&str, ParsedMove> {
    todo!()
}

pub fn parse_undo(input: &str) -> IResult<&str, ParsedMove> {
    let (input, _) = alt((tag("u"), tag("undo")));
    Ok((input, ParsedMove::Undo))
}

fn parse_left(input: &str) -> IResult<&str, u32> {
    nom::character::complete::u32(input)
}

fn parse_right(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list0(tag("+"), nom::character::complete::u32)(input)

}
