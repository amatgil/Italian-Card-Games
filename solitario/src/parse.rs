use nom::{
    IResult,
    bytes::complete::tag,
    branch::alt,
    error::ParseError,
    error::ErrorKind,
    error::VerboseErrorKind,
};
use nom::character::complete::u32 as p_u32;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub struct CustomError<I> {
    errors: Vec<CustomErrorKind<I>>
}


// TODO: This is temporary, parse_move should return the proper CustomError type. However, I am
// giving up for now on it
#[derive(thiserror::Error, Debug, Clone)]
#[error("error while parsing '{input}': '{reason}'")]
pub struct ParsingError {
    input: String,
    reason: String // this sucks
}

pub type CResult<I, T> = IResult<I, T, CustomError<I>>;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum CustomErrorKind<I> {
  #[error("standard nom error: {1:?}")]
  Nom(I, VerboseErrorKind),
  #[error("input had text left over after a successful parse")]
  InputHadLeftovers(I), // `undoo` doesn't count as `undo`
  #[error("repeated value found")]
  RepeatedSelection,    // can't move e.g. from pile 3 to 3
  #[error("at least one of the selected game piles was out of range: {0}")]
  OutOfRangePiles(usize), // There's only seven of them
  #[error("at least one of the selected ace piles was out of range: {0}")]
  OutOfRangeAces(usize),  // There's only four of them
  #[error("amount was too large (must be under u8::MAX: '{0}'")]
  AmountTooLarge(usize), // Amount must be under u8::MAX
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
    Quit,
    Cycle,
    RevealNextOfStack,
    MoveFromStackToPile(u8),
    MoveFromStackToAce(u8),
    MoveFromPileToPile {
        from: u8,
        to: u8,
        amount: u8,
    },
    MoveFromPileToAce { // Lowest card of pile only
        pile: u8,
        ace: u8,
    },
    MoveFromAceToPile { // Top card of ace only
        ace: u8,
        pile: u8,
    },
}

pub const SYNTAX_CHEATSHEET: &str = r#"| Action                                      | Syntax        |
|---------------------------------------------+---------------|
| Reveal next card in stack                   | `n` or `next` |
| Undo                                        | `u` or `undo` |
| Move top card in stack to pile X            | `s;X`         |
| Move top card in stack to ace X             | `s;aX`        |
| Move N cards from pile X to Y               | `mX;Y;N`      |
| Move lowest card from pile X to ace stack Y | `mX;aY`       |
| Move top card from ace stack Y to pile X    | `maY;X`       |
| Put all cards back on the stack, face down  | `cycle`       |
| Quit                                        | `q` or `quit` |"#;


/// Syntax:
///  | Action                                      | Syntax        |
///  |---------------------------------------------+---------------|
///  | Reveal next card in stack                   | `next`        |
///  | Undo                                        | `u` or `undo` |
///  | Move top card in stack to pile X            | `s;X`         |
///  | Move top card in stack to ace X             | `s;aX`        |
///  | Move N cards from pile X to Y               | `mX;Y;N`      |
///  | Move lowest card from pile X to ace stack Y | `mX;aY`       |
///  | Move top card from ace stack Y to pile X    | `maY;X`       |
///  | Put all cards back on the stack, face down  | `cycle`       |
///  | Quit                                        | `q` or `quit` |
pub fn parse_move(input: &str) -> Result<ParsedMove, ParsingError> {
    let original_input = input; // Copy ref

    let (input, res) = alt((parse_stack_revealing,
         parse_move_stack_to_pile,
         parse_move_stack_to_ace,
         parse_move_pile_to_pile,
         parse_move_pile_to_aces,
         parse_move_aces_to_pile,
         parse_undo,
         parse_cycle,
         parse_quit,
     ))(input.trim()).map_err(|e| ParsingError {
        input: original_input.to_string(),
        reason: e.to_string()
    })?;

    if !input.is_empty() {
        //Err(CustomError::new(CustomErrorKind::InputHadLeftovers(input)))
        Err(ParsingError {
            input: original_input.to_string(),
            reason: "input had leftovers".to_string()
        })
    } else {
        Ok(res)
    }
}

pub fn parse_move_prefix(input: &str) -> CResult<&str, ()> {
    let (input, _) = alt((tag("m;"), tag("m")))(input)?; // I keep instinctively typing `m;` so we're going to accept it too
    Ok((input, ()))
}

pub fn parse_stack_revealing(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = alt((tag("next"), tag("n")))(input)?; // Order is important
    Ok((input, ParsedMove::RevealNextOfStack))
}

pub fn parse_undo(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = alt((tag("undo"), tag("u")))(input)?; // Order is still important
    Ok((input, ParsedMove::Undo))
}

pub fn parse_cycle(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = tag("cycle")(input)?; // Order is still important
    Ok((input, ParsedMove::Cycle))
}

pub fn parse_quit(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = alt((tag("quit"), tag("q")))(input)?; // Order is still still important
    Ok((input, ParsedMove::Quit))
}

pub fn parse_move_stack_to_pile(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = tag("s")(input)?;
    let (input, _) = tag(";")(input)?;
    let (input, n) = p_u32(input)?;

    if n >= 7 {
        Err(CustomError::new(CustomErrorKind::OutOfRangePiles(n as usize)))
    } else {
        Ok((input, ParsedMove::MoveFromStackToPile(n as u8))) // Safe, is < 7
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
        Ok((input, ParsedMove::MoveFromStackToAce(n as u8))) // Safe, is < 4
    }
}

pub fn parse_move_pile_to_pile(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = parse_move_prefix(input)?;
    let (input, x) = p_u32(input)?;
    let (input, _) = tag(";")(input)?;   
    let (input, y) = p_u32(input)?;   
    let (input, _) = tag(";")(input)?;   
    let (input, n) = p_u32(input)?;   

    if x >= 7 || y >= 7 {
        let z = x.max(y);
        Err(CustomError::new(CustomErrorKind::OutOfRangePiles(z as usize)))
    } else if x == y {
        Err(CustomError::new(CustomErrorKind::RepeatedSelection))
    } else if n >= u8::MAX as u32 {
        Err(CustomError::new(CustomErrorKind::AmountTooLarge(n as usize)))
    } else {
        Ok((input, ParsedMove::MoveFromPileToPile {
            from: x as u8, // u32 -> u8 is ok because we know they're under 7
            to: y as u8,
            amount: n as u8,
        }))
    }

}

pub fn parse_move_pile_to_aces(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = parse_move_prefix(input)?;
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
            pile: pile as u8,
            ace: ace as u8,
        }))
    }
}

pub fn parse_move_aces_to_pile(input: &str) -> CResult<&str, ParsedMove> {
    let (input, _) = parse_move_prefix(input)?;
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
            ace: ace as u8,
            pile: pile as u8,
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

        ("m;0;1;3",  PM::MoveFromPileToPile { from: 0, to: 1, amount: 3}),
        ("m;1;2;3",  PM::MoveFromPileToPile { from: 1, to: 2, amount: 3}),
        ("m;a1;2",   PM::MoveFromAceToPile { ace: 1, pile: 2}),
        ("m;1;a2",   PM::MoveFromPileToAce { pile: 1, ace: 2}),
    ];

    let errs = [
        "m3;3;2",  "m;;3;4;2",  "nexttt",
        "sekjfhjkfhkjsflkjsdhfklsdjkf",
        "un", "und", "undoo", "nextt", "nxtt",
        "dlkjhflkjshglks",
        "m0;0;2", "m1;1;2",  "m2;2;2",  "m4;4;2",  "m5;5;2",  "m6;6;2",  // Repeated piles
        "m8;1;2", "m-1;1;2",  // Out of range
        "s;7",    "s;a4",     // Number too big
        "ma7;2",  "ma2;7",   "m7;a2", "m2;a7", "m7;2;3", "m2;7;3",
        "s;a-1",  "m2;4;-3", // Negatives
    ];

    for (inp, out) in ok_pairs {
        assert_eq!(
            parse_move(inp).map(|(_i, x)| x), // Fuck away the input
            Ok(out));
    }

    for inp in errs {
        dbg!(parse_move(inp), inp);
        assert!(parse_move(inp).is_err());
    }
}
