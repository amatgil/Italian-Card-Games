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

#[derive(Clone, Debug, Default, Copy, PartialEq, Eq)]
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
    pub who_is_first: PlayerKind,
    pub who_won_last_round: Turn,
    pub last_move: Option<Move>,
}

#[derive(Clone, Debug)]
pub struct Move {
    pub turn: Turn,
    pub card_played: Card,
    pub cards_taken: Option<Vec<Card>>,
}

#[derive(Clone, Debug)]
pub struct Match {
    pub turn: Turn,
    pub player_first: Player,
    pub player_shuffler: Player,
    pub deck: Vec<Card>,
    pub table: Vec<Card>
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Turn {
    #[default]
    First,
    Shuffler
}

#[derive(Clone, Debug, Default)]
pub struct PointTally {
    scope_first: usize,
    scope_shuf: usize,
    num_cards: Option<Turn>,
    num_denari: Option<Turn>,
    sette_bello: Turn,
    re_bello: Turn,
    napoli: Option<(Turn, usize)>,
    primiera: Option<Turn>,
}

impl PointTally {
    pub fn first_points(&self) -> usize {
        count_points(self, Turn::First)
    }
    pub fn shuf_points(&self) -> usize {
        count_points(self, Turn::Shuffler)
    }
}

fn count_points(tally: &PointTally, turn: Turn) -> usize {
    let mut p = 0;

    if turn == Turn::First { p += tally.scope_first }
    else                   { p += tally.scope_shuf }

    p += [
        tally.num_cards   == Some(turn),
        tally.num_denari  == Some(turn),
        tally.sette_bello == turn,
        tally.re_bello    == turn, 
        tally.primiera    == Some(turn),
    ].into_iter().filter(|&b| b).count();

    if let Some((napoli_turn, amount)) = tally.napoli {
        if napoli_turn == turn { p += amount }
    }

    p
}

impl Game {
    pub fn new() -> Game {
        Game {
            who_is_first: PlayerKind::Purple,
            curr_match: Match::new(),
            purple_points: 0,
            green_points:  0,
            who_won_last_round: Turn::First,
            last_move: None
        }
    }

    pub fn make_move<'a>(&'a mut self, mov: &'a str) -> Result<Option<Move>, MoveError> {
        let m = self.curr_match.make_move(mov)?;
        if let Some(move_made) = &m {
            self.who_won_last_round = move_made.turn;
        }
        Ok(m)
    }
    pub fn toggle_turn(&mut self) {
        self.curr_match.turn.toggle_turn()
    }
    pub fn winner(&self) -> Option<(String, usize, usize)> {
        let (purp, grep) = (self.purple_points, self.green_points);
        let (purple_win, green_win) = (Some((purple_text(), purp, grep)), Some((green_text(),  grep, purp)));

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

    pub fn is_match_over(&mut self) -> Option<PointTally> {
        if !self.curr_match.is_over() {
            None
        } else {
            self.give_table_to_last_taker();
            Some(self.curr_match.tally_final_points())
        }
    }

    pub fn give_table_to_last_taker(&mut self) {
        let player: &mut Player = match self.who_won_last_round {
            Turn::First    => &mut self.curr_match.player_first,
            Turn::Shuffler => &mut self.curr_match.player_shuffler,
        };

        for _ in 0..self.curr_match.table.len() {
            player.pile.push(self.curr_match.table.pop().unwrap());
        }
    }

    pub fn color_playing(&self) -> PlayerKind {
        use PlayerKind as PK;
        match (self.curr_match.turn, self.who_is_first) {
            (Turn::First,    PK::Purple) => PK::Purple,
            (Turn::First,    PK::Green)  => PK::Green,
            (Turn::Shuffler, PK::Purple) => PK::Green,
            (Turn::Shuffler, PK::Green)  => PK::Purple,
        }
    }
    pub fn toggle_whose_first(&mut self) {
        self.who_is_first = !self.who_is_first;
    }
    pub fn print_cards_of_curr_player(&self) {
        let cards = match self.curr_match.turn {
            Turn::First    => &self.curr_match.player_first.curr_hand,
            Turn::Shuffler => &self.curr_match.player_shuffler.curr_hand,
        };

        let s: String = cards.iter().enumerate().map(|(i, c)| format!("{c}({i})")).collect::<Vec<String>>().join(" ; ");
        println!("{s}");
    }
}

impl Card {
    pub fn new(suit: Suit, n: usize) -> Card {
        match n {
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
        self.deck.is_empty()
            && self.player_first.curr_hand.is_empty()
            && self.player_shuffler.curr_hand.is_empty()
    }

    /// Returns a Result, that means
    /// - Ok(Option<Turn>): Is Some if the last move was a take, None if it was placing on the table (for keeping track of the last person to take)
    /// - Err(...): Read the docs for MoveError
    pub fn make_move<'a>(&'a mut self, input: &'a str) -> Result<Option<Move>, MoveError> {
        let mov = Self::parse_move(input)?;
        let last_move;

        let player = match self.turn {
            Turn::First => &mut self.player_first,
            Turn::Shuffler => &mut self.player_shuffler,
        };

        let hand_card: Card = *player.curr_hand.get(mov.from)
            .ok_or(MoveError::OutOfRangeOfHand)?;

        if let Some(to_indices) = mov.to {
            let table_cards: Vec<&Card> = to_indices.iter()
                .map(|&i| self.table.get(i))
                .collect::<Option<Vec<&Card>>>()
                .ok_or(MoveError::OutOfRangeOfTable)?;

            if hand_card.number == CardNum::Numeric(1) {
                // We have an ace, we get everything (including itself)

                last_move = Some(Move {
                    card_played: hand_card,
                    cards_taken: Some(self.table.iter().map(|c| c.clone()).collect()),
                    turn: self.turn,
                });

                for _ in 0..self.table.len() {
                    player.pile.push(self.table.pop().unwrap());
                }
                player.pile.push(hand_card); // Don't forget the ace

                // Remove it from hand
                remove_elem_from_vec(&mut player.curr_hand, hand_card);

            } else if hand_card.value() == table_cards.iter().map(|c| c.value()).sum() {
                for card in &table_cards {
                    player.pile.push(**card);
                    player.pile.push(hand_card);
                }
                last_move = Some(Move {
                    card_played: hand_card,
                    cards_taken: Some(table_cards.iter().map(|&c| c.clone()).collect()),
                    turn: self.turn,
                });

                for i in to_indices.into_iter().rev() { self.table.remove(i); } // Remove them from the table
                remove_elem_from_vec(&mut player.curr_hand, hand_card);

                if self.table.is_empty() { // Do we have a scopa (non-ace)?
                    player.scope += 1;
                }

            } else {
                return Err(MoveError::MismatchedValues);
            }
        } else {
            // Place on table
            // TODO: make asso piglia tutto do the thingy instead of placing it when placing a card on the table (`N;` vs `tN` should be identical with an ace)
            last_move = Some(Move {
                card_played: hand_card,
                cards_taken: None,
                turn: self.turn,
            });
            self.table.push(hand_card);
            remove_elem_from_vec(&mut player.curr_hand, hand_card);
        }

        if player.curr_hand.is_empty() && !self.deck.is_empty() {
            // Redeal three cards from the deck
            for _ in 0..3 {
                let c = self.deck.pop().unwrap();
                player.curr_hand.push(c);
            }
        }
        Ok(last_move)
    }

    fn tally_final_points(&self) -> PointTally {
        let mut tally = PointTally::default();

        let fir = &self.player_first.pile;
        let shuf = &self.player_shuffler.pile;

        tally.scope_first = self.player_first.scope;
        tally.scope_shuf = self.player_shuffler.scope;

        // Number of cards
        match fir.len().cmp(&shuf.len()) {
            Ordering::Greater  => tally.num_cards = Some(Turn::First),
            Ordering::Equal    => tally.num_cards = None,
            Ordering::Less     => tally.num_cards = Some(Turn::Shuffler),
        }

        // Number of Denari (monee monee monee)
        match fir.iter().filter(|c| c.suit == Suit::Denari).count()
            .cmp(&shuf.iter().filter(|c| c.suit == Suit::Denari).count()) {
            Ordering::Greater  => tally.num_denari = Some(Turn::First),
            Ordering::Equal    => tally.num_denari = None,
            Ordering::Less     => tally.num_denari = Some(Turn::Shuffler),
        }

        // Who has 7 bello
        if fir.iter().any(|c| c == &Card::denari(7)) {
            tally.sette_bello = Turn::First;
        } else {
            tally.sette_bello = Turn::Shuffler;
        }

        // Who has king bello
        if fir.iter().any(|c| c == &Card::denari(10 /* Re */)) {
            tally.re_bello = Turn::First;
        } else {
            tally.re_bello = Turn::Shuffler;
        }

        // Napoli 
        if let Some(p) = check_napoli(fir) {
            tally.napoli = Some((Turn::First, p));
        } else if let Some(p) = check_napoli(shuf) {
            tally.napoli = Some((Turn::Shuffler, p));
        } else {
            tally.napoli = None;
        }

        // Primiera (7s thing (just counting))
        let mut i = 7;
        while i > 0 {
            match cards_with_value(i, fir).cmp(&cards_with_value(i, shuf)) {
                Ordering::Greater => {
                    tally.primiera = Some(Turn::First);
                    break;
                },
                Ordering::Equal   => i -= 1,
                Ordering::Less    => {
                    tally.primiera = Some(Turn::Shuffler);
                    break;
                }
            }
        }
        
        tally
    }

    fn parse_move(mov: &str) -> Result<ParsedMove, MoveError> {
        let (_, result) = parse_move_internal(mov).map_err(MoveError::ParseError)?;
        Ok(result)
    }
}

pub fn has_full_napoli(pila: &[Card]) -> bool {
    (1..=10).all(|i| pila.contains(&Card::denari(i)))
}

fn cards_with_value(target_value: usize, cards: &[Card]) -> usize {
    cards.iter().filter(|c| c.value() == target_value).count()
}

fn check_napoli(pila: &[Card]) -> Option<usize> {
    if [1, 2, 3].iter().all(|&i| pila.contains(&Card::denari(i))) {
        if !pila.contains(&Card::denari(4)) {
            Some(1)
        } else {
            let mut i = 4; // This goes all the way up, but players should only have up to 10/re
            while i < 10 && pila.contains(&Card::denari(i)) { i += 1 }
            Some(i)
        }
    } else {
        None
    }
    
}

fn remove_elem_from_vec<T>(v: &mut Vec<T>, elem: T) where T: PartialEq {
    let index = v.iter().position(|x| x == &elem).unwrap();
    v.remove(index);
}

struct ParsedMove {
    from: usize,
    to: Option<Vec<usize>>
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
            CardNum::Numeric(1) => write!(f, "A"),
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
            self.table.iter().enumerate().map(|(i, c)| format!("{c}({i})")).collect::<Vec<String>>().join(" ; "),
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
            PlayerKind::Purple => write!(f, "{}", purple_text()),
            PlayerKind::Green  => write!(f, "{}", green_text()),
        }
    }
}

impl std::ops::Not for PlayerKind {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Self::Purple => Self::Green,
            Self::Green  => Self::Purple,
        }
    }
}


impl Display for PointTally {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f,
               "=================
First's scope:\t\t\t{},
Shuf's scope:\t\t\t{},
Nº cards:\t\t\t{},
Nº denari\t\t\t{},
Sette bello:\t\t\t{},
Re bello:\t\t\t{},
Napoli:  \t\t\t{},
Primiera:\t\t\t{}
=================",
               self.scope_first, self.scope_shuf,
               self.num_cards.map(|n| n.to_string()).unwrap_or("Nobody".to_string()),
               self.num_denari.map(|n| n.to_string()).unwrap_or("Nobody".to_string()),
               self.sette_bello,
               self.re_bello,
               self.napoli.map(|(t, n)| format!("{t} ({n})")).unwrap_or("Nobody".to_string()),
               self.primiera.map(|n| n.to_string()).unwrap_or("Nobody".to_string()),
        )
    }
}

impl Default for Match {
    fn default() -> Self { Self::new() }
}
impl Default for Game {
    fn default() -> Self { Self::new() }
}


pub fn purple_text() -> String {
    format!("{0}[38;5;93mPurple{0}[0m", 27 as char)
}
pub fn green_text() -> String {
    format!("{0}[38;5;34mGreen{0}[0m", 27 as char)
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.cards_taken {
            None => write!(f, "{} placed {} on the table",
                           self.turn,
                           self.card_played),
            Some(tables) => write!(f, "{} took {} with {}",
                                   self.turn,
                                   tables.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("+"),
                                   self.card_played),
        }
    }
}

