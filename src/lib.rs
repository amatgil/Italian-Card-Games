
pub struct Player {
    cards: Vec<Card>, // Three or less held cards
}

pub struct Card {
    suit: Suit,
    number: CardNum
}

pub enum Suit {
    Denari,
    Coppe,
    Bastoni,
    Spade, 
}

pub enum CardNum {
    Numeric(usize),
    Fante,
    Cavallo,
    Re
}

pub struct Game {
    player_first: Player,
    player_shuffler: Player,
    deck: Vec<Card>,
    table: Vec<Card>
}

impl Card {
    fn value(&self) -> usize {
        match self {
            CardNum::Numeric(n) => n,
            CardNum::Fante      => 8,
            CardNum::Cavallo    => 9,
            CardNum::Re         => 10,
        }
    }

    fn shuffled_deck() -> Vec<Card> {
        let numbers = [CardNum::Numeric(0), CardNum::Numeric(1), CardNum::Numeric(2),
                       CardNum::Numeric(3), CardNum::Numeric(4), CardNum::Numeric(5),
                       CardNum::Numeric(6), CardNum::Numeric(7), CardNum::Fante,
                       CardNum::Cavallo,    CardNum::Re];

        let suits = [Suit::Denari, Suit::Coppe, Suit::Bastoni, Suit::Spade];

        let deck = Vec::new();
        for num in numbers {
            for suit in suits {
                deck.push(Card { num } )
            }
        }

        deck
    }
}
impl Game {
    fn new() -> Game {
        Game {
            player_first: Player { cards: vec![] },
            player_shuffler: Player { cards: vec![] },
            deck: Card::shuffled_deck(),
            table: vec![],
        }
    }
}
