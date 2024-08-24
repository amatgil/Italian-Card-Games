use nom::{
    IResult,
    bytes::complete::tag,
    multi::separated_list0,
    branch::alt,
    error::ParseError,
    error::ErrorKind,
    error::VerboseErrorKind,
};
use nom::character::complete::u32 as p_u32;

#[derive(Debug, PartialEq)]
pub struct CustomError<I> {
    errors: Vec<CustomErrorKind<I>>
}

pub type CResult<I, T> = IResult<I, T, CustomError<I>>;

#[derive(Debug, PartialEq)]
pub enum CustomErrorKind<I> {
  Nom(I, VerboseErrorKind),
  InputHadLeftovers(I), // `undoo` doesn't count as `undo`
  RepeatedSelection,    // can't move e.g. from pile 3 to 3
  OutOfRangePiles(usize), // There's only seven of them
  OutOfRangeAces(usize),  // There's only four of them
}

impl<I> ParseError<I> for CustomError<I> {
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    CustomError {
        errors: vec![CustomErrorKind::Nom(input, VerboseErrorKind::Nom(kind))]
    }
  }

  fn append(input: I, kind: ErrorKind, mut other: Self) -> Self {
      other.errors.push(CustomErrorKind::Nom(input, VerboseErrorKind::Nom(kind)));
      other
  }
}

impl<I> CustomError<I> {
    fn new(kind: CustomErrorKind<I>) -> nom::Err<CustomError<I>> {
        nom::Err::Error(CustomError { errors: vec![kind] })
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsedMove {
    Undo,
    RevealNextOfStack,
    MoveFromStackToPile(usize),
    MoveFromStackToAce(usize),
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
        ace: usize,
        pile: usize,
    },
}

/// Syntax:
///  | Action                                      | Syntax        |
///  |---------------------------------------------+---------------|
///  | Reveal next card in stack                   | `next`        |
///  | Undo                                        | `u` or `undo` |
///  | Move top card in stack to pile X            | `s;X`         |
///  | Move N cards from pile X to Y               | `mX;Y;N`      |
///  | Move lowest card from pile X to ace stack Y | `mX;aY`       |
///  | Move top card from ace stack Y to pile X    | `maY;X`       |
pub fn parse_move(input: &str) -> CResult<&str, ParsedMove> {
    let (input, res) = alt((parse_stack_revealing,
         parse_move_stack_to_pile,
         parse_move_stack_to_ace,
         parse_move_pile_to_pile,
         parse_move_pile_to_aces,
         parse_move_aces_to_pile,
         parse_undo
     ))(input.trim())?;

    if !input.is_empty() {
        Err(CustomError::new(CustomErrorKind::InputHadLeftovers(input)))
    } else {
        Ok((input, res))
    }
}

pub fn parse_stack_revealing(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = alt((tag("next"), tag("n")))(input)?; // Order is important
    Ok((input, ParsedMove::RevealNextOfStack))
}

pub fn parse_undo(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = alt((tag("undo"), tag("u")))(input)?; // Order is still important
    Ok((input, ParsedMove::Undo))
}

pub fn parse_move_stack_to_pile(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = tag("s")(input)?;
    let (input, _) = tag(";")(input)?;
    let (input, n) = p_u32(input)?;

    if n >= 7 {
        Err(CustomError::new(CustomErrorKind::OutOfRangePiles(n as usize)))
    } else {
        Ok((input, ParsedMove::MoveFromStackToPile(n as usize)))
    }
}

pub fn parse_move_stack_to_ace(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = tag("s")(input)?;
    let (input, _) = tag(";")(input)?;
    let (input, _) = tag("a")(input)?;
    let (input, n) = p_u32(input)?;

    if n >= 4 {
        Err(CustomError::new(CustomErrorKind::OutOfRangeAces(n as usize)))
    } else {
        Ok((input, ParsedMove::MoveFromStackToAce(n as usize)))
    }
}

pub fn parse_move_pile_to_pile(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = tag("m")(input)?;   
    let (input, x) = p_u32(input)?;
    let (input, _) = tag(";")(input)?;   
    let (input, y) = p_u32(input)?;   
    let (input, _) = tag(";")(input)?;   
    let (input, n) = p_u32(input)?;   

    if x >= 7 || y >= 7 {
        Err(CustomError::new(CustomErrorKind::OutOfRangePiles(n as usize)))
    } else {
        Ok((input, ParsedMove::MoveFromPileToPile {
            from: x as usize,
            to: y as usize,
            amount: n as usize,
        }))
    }

}

pub fn parse_move_pile_to_aces(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = tag("m")(input)?;   
    let (input, pile) = p_u32(input)?;
    let (input, _) = tag(";")(input)?;   
    let (input, _) = tag("a")(input)?;   
    let (input, ace) = p_u32(input)?;   


    if pile >= 7 {
        Err(CustomError::new(CustomErrorKind::OutOfRangePiles(pile as usize)))
    } else if ace >= 4 {
        Err(CustomError::new(CustomErrorKind::OutOfRangeAces(ace as usize)))
    } else {
        Ok((input, ParsedMove::MoveFromPileToAce {
            pile: pile as usize,
            ace: ace as usize,
        }))
    }
}

pub fn parse_move_aces_to_pile(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = tag("m")(input)?;   
    let (input, _) = tag("a")(input)?;   
    let (input, ace) = p_u32(input)?;
    let (input, _) = tag(";")(input)?;   
    let (input, pile) = p_u32(input)?;   

    if pile >= 7 {
        Err(CustomError::new(CustomErrorKind::OutOfRangePiles(pile as usize)))
    } else if ace >= 4 {
        Err(CustomError::new(CustomErrorKind::OutOfRangeAces(ace as usize)))
    } else {
        Ok((input, ParsedMove::MoveFromAceToPile {
            ace: ace as usize,
            pile: pile as usize,
        }))
    }
}

// ============== TESTS ================

#[test]
fn parsing_battery() {
    use ParsedMove as PM;
    let ok_pairs = [
        ("n",       PM::RevealNextOfStack),
        ("next",    PM::RevealNextOfStack),
        ("u",       PM::Undo),
        ("undo",    PM::Undo),
        ("s;0",     PM::MoveFromStackToPile(0)),
        ("s;1",     PM::MoveFromStackToPile(1)),
        ("s;2",     PM::MoveFromStackToPile(2)),
        ("s;3",     PM::MoveFromStackToPile(3)),
        ("s;6",     PM::MoveFromStackToPile(6)),
        ("s;a0",    PM::MoveFromStackToAce(0)),
        ("s;a1",    PM::MoveFromStackToAce(1)),
        ("s;a2",    PM::MoveFromStackToAce(2)),
        ("s;a3",    PM::MoveFromStackToAce(3)),
        ("m0;1;3",  PM::MoveFromPileToPile { from: 0, to: 1, amount: 3}),
        ("m1;2;3",  PM::MoveFromPileToPile { from: 1, to: 2, amount: 3}),
        ("ma1;2",   PM::MoveFromAceToPile { ace: 1, pile: 2}),
        ("m1;a2",   PM::MoveFromPileToAce { pile: 1, ace: 2}),
    ];

    let errs = [
        "nexttt",
        "sekjfhjkfhkjsflkjsdhfklsdjkf",
        "un",
        "und",
        "undoo",
        "nextt",
        "nxtt",
        "dlkjhflkjshglks",
        "m;1;1;2",  // Repeated piles
        "m;8;1;2",  // Out of range
        "m;-1;1;2", // Out of range
        "s;7",      // Number too big
        "s;a4",     // Number too big
        "s;a-1",     // Number too big
        "ma7;2",
        "ma2;7",
        "m7;a2",
        "m2;a7",
        "m7;2;3",
        "m2;7;3",
        "m2;4;-3",
    ];

    for (inp, out) in ok_pairs {
        assert_eq!(
            parse_move(inp).map(|(_i, x)| x), // Fuck away the input
            Ok(out));
    }

    for inp in errs {
        dbg!(parse_move(inp), inp);
        assert!(parse_move(inp).is_err())
    }
}


// Syntax:
//  | Action                                      | Syntax        |
//  |---------------------------------------------+---------------|
//  | Reveal next card in stack                   | `next`        |
//  | Undo                                        | `u` or `undo` |
//  | Move top card in stack to pile X            | `s;X`         |
//  | Move N cards from pile X to Y               | `mX;Y;N`      |
//  | Move lowest card from pile X to ace stack Y | `mX;aY`       |
//  | Move top card from ace stack Y to pile X    | `maY;X`       |
