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

#[derive(Clone, Copy, Debug)]
pub enum CardNum {
    Numeric(usize),
    Fante,
    Cavallo,
    Re
}

#[derive(Clone, Debug)]
pub struct Game {
    player_purple: usize, // Host, probably
    player_green: usize,
}

#[derive(Clone, Debug)]
pub struct Match {
    pub player_first: Player,
    pub player_shuffler: Player,
    pub deck: Vec<Card>,
    pub table: Vec<Card>
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

        Match { player_first, player_shuffler, deck, table }
    }
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
