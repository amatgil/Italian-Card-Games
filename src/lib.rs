use std::fmt::{Display, Formatter};
use std::fmt::Debug;
use std::cmp::Ordering;

mod parse_move;
use crate::parse_move::*;

#[derive(Clone, Debug, Default)]
pub struct Player {
    pub curr_hand: Vec<Card>, // Three or less held cards
    pub pile: Vec<Card>,      // Cards that they've won
    pub scope: usize,         // nº of scope obtained
}

#[derive(Clone, Debug, Default, Copy)]
pub enum PlayerKind {
    #[default]
    Purple,
    Green
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub number: CardNum
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    pub purple_points: usize, // Host, probably
    pub green_points: usize,
    pub curr_match: Match,
    pub whose_first: PlayerKind,
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
            whose_first: PlayerKind::Purple,
            curr_match: Match::new(),
            purple_points: 0,
            green_points:  0,
        }
    }

    pub fn make_move<'a>(&'a mut self, mov: &'a str) -> Result<(), MoveError> {
        self.curr_match.make_move(mov)
    }
    pub fn toggle_turn(&mut self) {
        self.curr_match.turn.toggle_turn()
    }
    pub fn winner(&self) -> Option<(&str, usize, usize)> {
        let (purp, grep) = (self.purple_points, self.purple_points);
        let (purple_win, green_win) = (Some(("Purple", purp, grep)), Some(("Green",  grep, purp)));

        match (purp, grep) {
            (0..=20, 0..=20)  => None,
            (21..,   0..=20)  => purple_win,
            (0..=20, 21..  )  => green_win,
            (p, g)            => match p.cmp(&g) {
                Ordering::Less    => green_win,
                Ordering::Equal   => None,
                Ordering::Greater => purple_win,
            }
        }
    }

    pub fn is_match_over(&self) -> Option<(usize, usize)> {
        if !self.curr_match.is_over() {
            None
        } else {
            let (first_p, shuffler_p) = self.curr_match.count_final_points();
            todo!()
        }
    }
    pub fn color_playing(&self) -> PlayerKind {
        use PlayerKind as PK;
        match (self.curr_match.turn, self.whose_first) {
            (Turn::First,    PK::Purple) => PK::Purple,
            (Turn::Shuffler, PK::Purple) => PK::Green,
            (Turn::First,    PK::Green)  => PK::Green,
            (Turn::Shuffler, PK::Green)  => PK::Purple,
        }
    }
    pub fn toggle_whose_first(&mut self) {
        use PlayerKind as PK;
        match self.whose_first {
            PK::Purple => self.whose_first = PK::Green,
            PK::Green => self.whose_first = PK::Purple,
        }
    }
    pub fn print_cards_of_curr_player(&self) {
        let cards = match self.curr_match.turn {
            Turn::First    => &self.curr_match.player_first.curr_hand,
            Turn::Shuffler => &self.curr_match.player_shuffler.curr_hand,
        };

        let s: String = cards.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" ; ");
        println!("{s}");
    }
}

impl Card {
    pub fn new(suit: Suit, n: usize) -> Card {
        match n { // "dc" as in "Denari card"
            1..=7 => Card { suit, number: CardNum::Numeric(n) },
            8     => Card { suit, number: CardNum::Fante },
            9     => Card { suit, number: CardNum::Cavallo },
            10    => Card { suit, number: CardNum::Re },
            _     => panic!("Tried to make a card that's greater than 10"),
        }
    }

    pub fn denari(n: usize) -> Card {
        match n { 
            1..=7 => Card { suit: Suit::Denari, number: CardNum::Numeric(n) },
            8     => Card { suit: Suit::Denari, number: CardNum::Fante },
            9     => Card { suit: Suit::Denari, number: CardNum::Cavallo },
            10    => Card { suit: Suit::Denari, number: CardNum::Re },
            _     => panic!("Tried to make a card that's greater than 10")
        }
    }

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

    pub fn is_over(&self) -> bool {
        self.table.is_empty()
            && self.deck.is_empty()
            && self.player_first.curr_hand.is_empty()
            && self.player_shuffler.curr_hand.is_empty()
    }

    pub fn make_move<'a>(&'a mut self, input: &'a str) -> Result<(), MoveError> {
        let mov = Self::parse_move(input)?;

        let player = match self.turn {
            Turn::First => &mut self.player_first,
            Turn::Shuffler => &mut self.player_shuffler,
        };

        let hand_card: Card = player.curr_hand.get(mov.from)
            .ok_or(MoveError::OutOfRangeOfHand)?.clone();

        let table_cards: Vec<&Card> = mov.to.iter()
            .map(|&i| self.table.get(i))
            .collect::<Option<Vec<&Card>>>()
            .ok_or(MoveError::OutOfRangeOfTable)?;

        if hand_card.number == CardNum::Numeric(1) {
            // We have an ace, we get everything (including itself)
            for _ in 0..self.table.len() {
                player.pile.push(self.table.pop().unwrap());
            }
            player.pile.push(hand_card); // Don't forget the ace

            // Remove it from hand
            remove_elem_from_vec(&mut player.curr_hand, hand_card);

            Ok(())
        } else if hand_card.value() == table_cards.iter().map(|c| c.value()).sum() {
            for card in &table_cards {
                player.pile.push(**card);
                player.pile.push(hand_card);
            }
            for i in mov.to { self.table.remove(i); } // Remove them from the table
            remove_elem_from_vec(&mut player.curr_hand, hand_card);


            if self.table.len() == 0 { // Do we have a scopa (non-ace)?
                player.scope += 1;
            }

            Ok(())
        } else {
            Err(MoveError::MismatchedValues)
        }
    }

    fn count_final_points(&self) -> (usize, usize) {
        let mut fir_points = 0;
        let mut shuf_points = 0;

        let fir = &self.player_first.pile;
        let shuf = &self.player_shuffler.pile;

        // Number of cards
        match fir.len().cmp(&shuf.len()) {
            Ordering::Greater  => fir_points += 1,
            Ordering::Equal    => {},
            Ordering::Less     => shuf_points += 1,
        }

        // Number of Denari (monee monee monee)
        match fir.iter().filter(|c| c.suit == Suit::Denari).count()
            .cmp(&shuf.iter().filter(|c| c.suit == Suit::Denari).count()) {
            Ordering::Greater  => fir_points += 1,
            Ordering::Equal    => {},
            Ordering::Less     => shuf_points += 1,
        }

        // Who has 7 bello
        if fir.iter().position(|c| c == &Card::denari(7)).is_some() {
            fir_points += 1;
        } else {
            shuf_points += 1;
        }

        // Who has king bello
        if fir.iter().position(|c| c == &Card::denari(10 /* Re */)).is_some() {
            fir_points += 1;
        } else {
            shuf_points += 1;
        }

        // Napoli 
        check_napoli(&fir, &mut fir_points);
        check_napoli(&shuf, &mut shuf_points);

        // Primiera (7s thing (just counting))
        let mut i = 7;
        while i > 0 {
            match cards_with_value(i, fir).cmp(&cards_with_value(i, shuf)) {
                Ordering::Greater => {
                    fir_points += 1;
                    break;
                },
                Ordering::Equal   => i -= 1,
                Ordering::Less    => {
                    shuf_points += 1;
                    break;
                }
            }
        }
        
        (fir_points, shuf_points)
    }
    // I'm too lazy to write another enum that's a subset of MoveError AND THEN impl From<>. I can just return it
    fn parse_move(mov: &str) -> Result<ParsedMove, MoveError> {
        let (input, result) = parse_move_internal(mov).map_err(|e| MoveError::ParseError(e))?;
        Ok(result)
    }
}

pub fn has_full_napoli(pila: &[Card]) -> bool {
    (1..=10).all(|i| pila.contains(&Card::denari(i)))
}

fn cards_with_value(target_value: usize, cards: &[Card]) -> usize {
    cards.iter().filter(|c| c.value() == target_value).count()
}

fn check_napoli(pila: &[Card], points: &mut usize) {
    if [1, 2, 3].iter().all(|&i| pila.contains(&Card::denari(i))) {
        if !pila.contains(&Card::denari(4)) { *points += 1; }
        else {
            let mut i = 4; // This goes all the way up, but players should only have up to 10/re
            while pila.contains(&Card::denari(i)) { i += 1 }
            *points += i;
        }
    }
}

fn remove_elem_from_vec<T>(v: &mut Vec<T>, elem: T) where T: PartialEq {
    let index = v.iter().position(|x| x == &elem).unwrap();
    v.remove(index);
}

struct ParsedMove {
    from: usize,
    to: Vec<usize>
}

#[derive(Debug)]
pub enum MoveError<'a> {
    /// Move could not be parsed
    ParseError(nom::Err<nom::error::Error<&'a str>>),
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

impl Display for Turn {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Turn::First    => write!(f, "First"),
            Turn::Shuffler => write!(f, "Shuffler"),
        }
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        
        let s = format!(
"-------------------
Turn: '{}'
Deck has '{}' cards ('{}' turns left)
First has '{}' cards
Shuffler has '{}' cards
Table has cards: '{}'
-------------------",
            self.turn,
            self.deck.len(), self.deck.len() / 6,
            self.player_first.curr_hand.len(),
            self.player_shuffler.curr_hand.len(),
            self.table.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" ; "),
        );
        write!(f, "{s}")
    }
}


impl Turn {
    fn toggle_turn(&mut self) {
        match self {
            Self::First => *self = Self::Shuffler,
            Self::Shuffler => *self = Self::First,
        }
    }
}

impl Display for PlayerKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            PlayerKind::Purple => write!(f, "Purple"),
            PlayerKind::Green  => write!(f, "Green"),
        }
    }
}
