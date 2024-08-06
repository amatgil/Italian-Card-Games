use std::fmt::{Display, Formatter};
use std::fmt::Debug;

#[derive(Clone, Debug, Default)]
pub struct Player {
    pub curr_hand: Vec<Card>, // Three or less held cards
    pub pile: Vec<Card>,      // Cards that they've won
    pub scope: usize,         // nº of scope obtained
}

#[derive(Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub number: CardNum
}

#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Denari,
    Coppe,
    Bastoni,
    Spade, 
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardNum {
    Numeric(usize),
    Fante,
    Cavallo,
    Re
}

#[derive(Clone, Debug)]
pub struct Game {
    purple_points: usize, // Host, probably
    green_points: usize,
    curr_match: Match
}

#[derive(Clone, Debug)]
pub struct Match {
    pub turn: Turn,
    pub player_first: Player,
    pub player_shuffler: Player,
    pub deck: Vec<Card>,
    pub table: Vec<Card>
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Turn {
    #[default]
    First,
    Shuffler
}

impl Game {
    pub fn new() -> Game {
        Game {
            curr_match: Match::new(),
            purple_points: 0,
            green_points:  0,
        }
    }
}

impl Card {
    pub fn value(&self) -> usize {
        match self.number {
            CardNum::Numeric(n) => n,
            CardNum::Fante      => 8,
            CardNum::Cavallo    => 9,
            CardNum::Re         => 10,
        }
    }

    pub fn shuffled_deck() -> Vec<Card> {
        let numbers = [CardNum::Numeric(1), CardNum::Numeric(2), CardNum::Numeric(3),
                       CardNum::Numeric(4), CardNum::Numeric(5), CardNum::Numeric(6),
                       CardNum::Numeric(7), CardNum::Fante, CardNum::Cavallo, CardNum::Re];

        let suits = [Suit::Denari, Suit::Coppe, Suit::Bastoni, Suit::Spade];

        let mut deck = Vec::with_capacity(numbers.len()*suits.len());
        for number in numbers {
            for suit in suits {
                deck.push(Card { number, suit  } )
            }
        }

        // Shuffle the deck (Fisher-Yates my beloved)
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for i in (1..deck.len()).rev() {
            let j = rng.gen_range(0..=i);
            deck.swap(i, j);
        }

        deck
    }
}


impl Match {
    pub fn new() -> Match {
        let mut deck = Card::shuffled_deck();

        let mut player_first = Player::default();
        let mut player_shuffler = Player::default();

        for _ in 0..3 {
            let (c1, c2) = (deck.pop().unwrap(), deck.pop().unwrap());

            player_first.curr_hand.push(c1);
            player_shuffler.curr_hand.push(c2);
        }

        let mut table = Vec::new();
        for _ in 0..4 {
            let c = deck.pop().unwrap();
            table.push(c);
        }

        Match { player_first, player_shuffler, deck, table, turn: Turn::First }
    }

    pub fn make_move(&mut self, input: &str) -> Result<(), MoveError> {
        let mov = self.parse_move(input)?;

        let player = match self.turn {
            Turn::First => &mut self.player_first,
            Turn::Shuffler => &mut self.player_shuffler,
        };

        let hand_card: &Card = player.curr_hand.get(mov.from)
            .ok_or(MoveError::OutOfRangeOfHand)?;

        let table_cards: Vec<&Card> = mov.to.iter()
            .map(|&i| self.table.get(i))
            .collect::<Option<Vec<&Card>>>()
            .ok_or(MoveError::OutOfRangeOfTable)?;

        if hand_card.number == CardNum::Numeric(1) {
            // We have an ace, we get everything (including itself)
            for i in 0..self.table.len() {
                player.pile.push(self.table.pop().unwrap());
            }
            player.pile.push(*hand_card); // Don't forget the ace
            Ok(())
        } else if hand_card.value() == table_cards.iter().map(|c| c.value()).sum() {
            for card in &table_cards {
                player.pile.push(**card);
                player.pile.push(*hand_card);
            }
            for i in mov.to { self.table.remove(i); } // Remove them from the table

            if self.table.len() == 0 { // Do we have a scopa (non-ace)?
                player.scope += 1;
            }
            Ok(())
        } else {
            Err(MoveError::MismatchedValues)
        }
    }

    // I'm too lazy to write another enum that's a subset of MoveError AND THEN impl From<>. I can just return it
    fn parse_move(&self, mov: &str) -> Result<ParsedMove, MoveError> {
        todo!()
    }

}

struct ParsedMove {
    from: usize,
    to: Vec<usize>
}

pub enum MoveError {
    /// Move could not be parsed
    ParseError,
    /// Index isn't a base 10 single-char digit
    InvalidDigit,
    /// When you try to play an ace but you don't have it
    InvalidAce,  
    /// Values don't match
    MismatchedValues,
    /// An addition did not yield the expected value
    AdditionDoesntCheckOut,
    /// The hand index was invalid
    OutOfRangeOfHand,
    /// At least one of the table indices was out of range 
    OutOfRangeOfTable,
}

impl Display for CardNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CardNum::Numeric(n) => write!(f, "{n}"),
            CardNum::Fante      => write!(f, "🧍"),
            CardNum::Cavallo    => write!(f, "🐴"),
            CardNum::Re         => write!(f, "👑"),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Suit::Denari  => write!(f, "💲"),
            Suit::Coppe   => write!(f, "🏆"),
            Suit::Bastoni => write!(f, "🪵"),
            Suit::Spade   => write!(f, "⚔️"),
        }
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}", self.number, self.suit)
    }
}
impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}", self.number, self.suit)
    }
}
