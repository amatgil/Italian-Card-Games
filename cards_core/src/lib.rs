use std::fmt::{Display, Formatter};
use std::fmt::Debug;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub number: CardNum
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Suit {
    Denari,
    Coppe,
    Bastoni,
    Spade, 
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardNum {
    Numeric(u8),
    Fante,
    Cavallo,
    Re
}

// [Bottom of deck .... Top of deck]
// Front is bottom, back is top
// | Internal | API |
// |----------+-----|
// | Front    | Bot |
// | Back     | Top |
#[derive(Clone, Debug, Default)]
pub struct Deck(pub VecDeque<Card>);

impl Deck {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }
    pub fn top(&self) -> Option<&Card> {
        self.0.back()
    }
    pub fn bottom(&self) -> Option<&Card> {
        self.0.front()
    }
    pub fn take_from_top(&mut self) -> Option<Card> {
        self.0.pop_back()
    }
    pub fn push_to_top(&mut self, c: Card) {
        self.0.push_back(c);
    }
    pub fn take_from_bottom(&mut self) -> Option<Card> {
        self.0.pop_front()
    }
    pub fn push_to_bottom(&mut self, c: Card) {
        self.0.push_front(c);
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn move_all_cards_to(&mut self, dest: &mut Deck) {
        for card in self.0.drain(..) {
            dest.push_to_bottom(card)
        }
    }
}

impl Card {
    pub fn new_it(suit: Suit, n: usize) -> Card {
        match n {
            1..=7 => Card { suit, number: CardNum::Numeric(n as u8) },
            8     => Card { suit, number: CardNum::Fante },
            9     => Card { suit, number: CardNum::Cavallo },
            10    => Card { suit, number: CardNum::Re },
            _     => panic!("Tried to make a card that's greater than 10"),
        }
    }
    pub fn new_fr(suit: Suit, n: usize) -> Card {
        match n {
            1..=10 => Card { suit, number: CardNum::Numeric(n as u8) },
            11     => Card { suit, number: CardNum::Fante },
            12     => Card { suit, number: CardNum::Cavallo },
            13     => Card { suit, number: CardNum::Re },
            _      => panic!("Tried to make a card that's greater than 10"),
        }
    }

    pub fn denari(n: usize) -> Card {
        match n { 
            1..=7 => Card { suit: Suit::Denari, number: CardNum::Numeric(n as u8) },
            8     => Card { suit: Suit::Denari, number: CardNum::Fante },
            9     => Card { suit: Suit::Denari, number: CardNum::Cavallo },
            10    => Card { suit: Suit::Denari, number: CardNum::Re },
            _     => panic!("Tried to make a card that's greater than 10")
        }
    }

    /// Assuming italian standard value counting
    pub fn value(&self) -> usize {
        match self.number {
            CardNum::Numeric(n) => n as usize,
            CardNum::Fante      => 8,
            CardNum::Cavallo    => 9,
            CardNum::Re         => 10,
        }
    }

    pub fn value_fr(&self) -> usize {
        match self.number {
            CardNum::Numeric(n) => n as usize,
            CardNum::Fante      => 11,
            CardNum::Cavallo    => 12,
            CardNum::Re         => 13,
        }
    }

    pub fn shuffled_basic_deck() -> Deck {
        Self::shuffled_deck(
            &[CardNum::Numeric(1), CardNum::Numeric(2), CardNum::Numeric(3),
            CardNum::Numeric(4), CardNum::Numeric(5), CardNum::Numeric(6),
            CardNum::Numeric(7), CardNum::Fante, CardNum::Cavallo, CardNum::Re])
    }
    pub fn shuffled_french_deck() -> Deck {
        Self::shuffled_deck(
            &[CardNum::Numeric(1), CardNum::Numeric(2), CardNum::Numeric(3),
            CardNum::Numeric(4), CardNum::Numeric(5), CardNum::Numeric(6),
            CardNum::Numeric(7), CardNum::Numeric(8), CardNum::Numeric(9), 
            CardNum::Numeric(10), CardNum::Fante, CardNum::Cavallo, CardNum::Re])
    }

    fn shuffled_deck(numbers: &[CardNum]) -> Deck {
        let suits = [Suit::Denari, Suit::Coppe, Suit::Bastoni, Suit::Spade];

        let mut deck = VecDeque::with_capacity(numbers.len()*suits.len());
        for number in numbers {
            for suit in suits {
                deck.push_back(Card { number: *number, suit  } )
            }
        }

        // Shuffle the deck (Fisher-Yates my beloved)
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for i in (1..deck.len()).rev() {
            let j = rng.gen_range(0..=i);
            deck.swap(i, j);
        }

        Deck(deck)
    }
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


impl std::ops::Deref for Deck {
    type Target = VecDeque<Card>;
    fn deref(&self) -> &VecDeque<Card> { &self.0 }
}
impl std::ops::DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut VecDeque<Card> { &mut self.0 }
}
