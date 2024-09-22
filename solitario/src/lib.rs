use cards_core::*;

mod parse;
pub use parse::*;

pub mod solver;
pub use solver::*;

pub use parse::SYNTAX_CHEATSHEET;

const RED_SUITS: [Suit; 2]   = [Suit::Denari, Suit::Spade];
const BLACK_SUITS: [Suit; 2] = [Suit::Coppe, Suit::Bastoni];

pub const UNKNOWN_CARD: &str = "---";

#[derive(Clone, Debug)]
pub struct Table {
    piles: [GamePile; 7],
    stack: Deck,
    passed_stack: Deck,
    aces: [AcePile; 4],
    moves: usize,
}

#[derive(Clone, Debug, Default)]
struct GamePile {
    cards: Vec<Card>,
    revealed: u8,  // how many cards of this pile have been revealed
}
#[derive(Clone, Debug, Default)]
struct AcePile {
    cards: Vec<Card>,
}



impl GamePile {
    /// Get the card that's closest to the top of the table (as in, has the highest value)
    /// Assumes that `self.revealed > 0`
    fn _get_head_of_revealed(&self) -> Option<&Card> {
        if self.cards.is_empty() { None }
        else {
            Some(&self.cards[self.cards.len() - self.revealed as usize])
        }
    }

    /// Get the card that's closest to the bottom of the table (as in, has the lowest value)
    /// Assumes the last card is always revealed (unless the len is 0, in which case it's None)
    fn get_tail_of_revealed(&self) -> Option<&Card> {
        self.cards.iter().last() // O(n) but they won't even get to 8, it's whatever
    }
    
    /// Get nth revealed (0 is lowest value, 1 is closer towards the K, etc.)
    /// `n` is the index, like in `.get` or indexing methods
    fn get_nth_revealed(&self, n: usize) -> Option<&Card> {
        dbg!(self.revealed, &self.cards, n);
        if (self.revealed as usize) < n || self.cards.len() <= n { return None; }

        dbg!(
        self.cards.iter().rev().nth(n) // O(n) also
        )
    }

    fn pop_tail_of_revealed(&mut self) -> Option<Card> {
        if self.revealed == 0 || self.cards.is_empty() { return None; }
        if self.revealed > 1 { self.revealed -= 1}
        self.cards.pop()
    }

    fn add_card(&mut self, card: Card) -> Result<(), IllegalGamePileAdd> {
        let my_last = self.cards.iter().last();
        if legality_check(&card, my_last) {
            self.add_card_unchecked(card);
            Ok(())
        } else {
            Err(IllegalGamePileAdd)
        }
    }

    fn add_card_unchecked(&mut self, card: Card) {
        self.cards.push(card);
        self.revealed += 1;
    }
}

impl AcePile {
    fn add_card(&mut self, card: Card) -> Result<(), IllegalAcePileAdd> {
        match (self.cards.iter().last(), card) {
            (None, Card { number: CardNum::Numeric(1), ..}) => self.add_card_unchecked(card),
            (Some(a@Card { suit: s_a, .. }), b@Card { suit: s_b, .. }) if *s_a == s_b && a.value_fr() + 1 == b.value_fr() => self.add_card_unchecked(card),
            _ => return Err(IllegalAcePileAdd),
        }
        Ok(())
    }
    fn add_card_unchecked(&mut self, card: Card) {
        self.cards.push(card);
    }
    fn top(&self) -> Option<&Card> {
        self.cards.iter().last()
    }
    fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[derive(thiserror::Error, Debug, Clone)]
#[error("attempted to add a card on an ace pile illegally")]
pub struct IllegalAcePileAdd;

#[derive(thiserror::Error, Debug, Clone)]
#[error("attempted to add a card on a game pile illegally")]
pub struct IllegalGamePileAdd;

#[derive(thiserror::Error, Debug, Clone)]
pub enum MoveMakingError {
    #[error("could not parse move: {0}")]
    Parsing(#[from] ParsingError), // TODO: make this (and the backend in src/parse.rs) be done properly (i gave up and am using a String for now, but the foundation is laid)
    #[error("{0}")]
    IllegalAcePileAdd(#[from] IllegalAcePileAdd),
    #[error("{0}")]
    IllegalGamePileAdd(#[from] IllegalGamePileAdd),
    #[error("game pile has no revealed cards to use")]
    GamePileHasNoRevealed,
    #[error("ace pile has no cards")]
    AcePileIsEmpty,
    #[error("stack has no uncovered cards, you may cycle it with `next`")]
    StackIsEmpty,
    #[error("while moving the game piles: {0}")]
    MovingGamePile(#[from] GamePileMovingError),
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum GamePileMovingError {
    #[error("pile has no revealed cards to use")]
    PileHasNoRevealed,
    #[error("move was illegal")]
    IllegalMove,
    #[error("pile out of range")]
    PileOutOfRange,
    #[error("specified amount was zero")]
    AmountWasZero,
    #[error("attempting to move more cards than are revealed")]
    NotEnoughRevealedCards(u8),
}

impl Table {
    pub fn new() -> Self {
        let mut deck = Card::shuffled_french_deck();
        let mut piles = std::array::from_fn(|_i| GamePile::default());
        for (p, pile) in piles.iter_mut().enumerate() {
            pile.revealed = 1;
            for _ in 0..p+1 {
                let card = deck.take_from_top().expect("Deck cannot be empty, we don't deal all cards");
                pile.cards.push(card);
            }
        }

        Self { piles,
               stack: deck,
               passed_stack: Deck::new(),
               aces: std::array::from_fn(|_i| AcePile::default()),
               moves: 0
        }
    }

    pub fn make_move(&mut self, m: &str) -> Result<(), MoveMakingError> {
        use ParsedMove as PM;
        match parse_move(m)? {
            PM::Undo => {
                println!("\x1B[1;41mUndoing is not yet implemented\x1B[0m");
            },
            PM::Quit => {
                println!("quit signal received, quitting...");
                std::process::exit(0);
            },
            PM::RevealNextOfStack => {
                if self.stack.is_empty() {
                    std::mem::swap(&mut self.stack, &mut self.passed_stack);
                } else {
                    let c = self.stack.take_from_top().expect("We're in the else branch, this can't fail"); 
                    self.passed_stack.push_to_bottom(c); // we push to bottom because we'll mem::swap when the stack runs out
                }
            },
            PM::MoveFromStackToPile(p) => {
                if self.stack.is_empty() { return Err(MoveMakingError::StackIsEmpty) }
                let c = self.stack.top().expect("We're on the branch where this is safe");
                self.piles[p as usize].add_card(*c)?;
                let _ = self.stack.take_from_top().expect("Same reason");
                if !self.passed_stack.is_empty() {
                    self.stack.push_to_top(self.passed_stack.take_from_bottom().unwrap());
                }
            },
            PM::MoveFromStackToAce(a) => {
                if self.stack.is_empty() { return Err(MoveMakingError::StackIsEmpty) }
                let c = self.stack.top().expect("We're on the branch where this is safe");
                self.aces[a as usize].add_card(*c)?;
                let _ = self.stack.take_from_top().expect("Same reason");

                if !self.passed_stack.is_empty() {
                    self.stack.push_to_top(self.passed_stack.take_from_bottom().unwrap());
                }
            },
            PM::MoveFromPileToPile { from, to, amount } => {
                self.move_pile(from, to, amount)?;
            },
            PM::MoveFromPileToAce { pile, ace } => {
                let card = self.piles[pile as usize].get_tail_of_revealed().ok_or(MoveMakingError::GamePileHasNoRevealed)?;
                self.aces[ace as usize].add_card(*card)?;
                let _ = self.piles[pile as usize].pop_tail_of_revealed();
            },
            PM::MoveFromAceToPile { ace, pile } => {
                let card = self.aces[ace as usize].top().ok_or(MoveMakingError::AcePileIsEmpty)?;
                self.piles[pile as usize].add_card(*card)?;
                let _ = self.aces[ace as usize].pop();
            },
            PM::Cycle => {
                while !self.stack.is_empty() { self.make_move("next")?; }

                if self.moves > 0 { self.moves -= 1; } // Uncount the `cycle` command, it's unintuitive and wrong
            }
        }
        self.moves += 1;
        Ok(())
    }
    pub fn move_pile(&mut self, from_idx: u8, to_idx: u8, amount: u8) -> Result<(), GamePileMovingError> {
        if from_idx >= 7 || to_idx >= 7 { return Err(GamePileMovingError::PileOutOfRange) }; 

        // We clone because we can't `&mut` them both at once, we'll reassign back if we're on the
        // happy path
        let mut from = self.piles[from_idx as usize].clone();
        let mut to   = self.piles[to_idx as usize].clone();

        if amount == 0 { return Err(GamePileMovingError::AmountWasZero) }
        let from_base = from.get_nth_revealed(amount as usize - 1).ok_or(GamePileMovingError::PileHasNoRevealed)?;
        let to_tail   = to.get_tail_of_revealed();


        
        if legality_check(from_base, to_tail) {
            eprintln!("Legality/King check passed: {:?} and {:?}", from_base, to_tail);
            if amount > from.revealed { return Err(GamePileMovingError::NotEnoughRevealedCards(amount)) };

            let removal_index = from.cards.len() - amount as usize; // len varies so we store it here
            for _ in 0..amount {
                dbg!(&from.cards);
                let c = from.cards.remove(removal_index);
                to.cards.push(c);
            }


            to.revealed += amount;
            from.revealed -= amount;

            if from.revealed == 0 { from.revealed = 1 }

            // We were on the happy path, we must reassign back
            self.piles[from_idx as usize] = from;
            self.piles[to_idx as usize] = to;
            Ok(())
        } else {
            Err(GamePileMovingError::IllegalMove) 
        }
    }
    pub fn has_won(&self) -> bool {
        self.piles.iter().map(|p| p.cards.len() == p.revealed as usize).all(|b| b)
    }
}


/// Denari and spade are red, coppe and bastoni are black. They must alternate.
/// If the base is None, added must be (any) K. If not, their values must be sequential with
/// alternating suits.
fn legality_check(added: &Card, base_opt: Option<&Card>) -> bool {
    dbg!(added, base_opt);
    if let Some(base) = base_opt {
        (added.value_fr() + 1 == base.value_fr()) 
            && !((RED_SUITS.contains(&base.suit) && RED_SUITS.contains(&added.suit))
                 || (BLACK_SUITS.contains(&base.suit) && BLACK_SUITS.contains(&added.suit)))

    } else {
        added.number == CardNum::Re 
    } 

}

use std::fmt::Display;
use std::fmt::Formatter;

fn print_card_fr(c: &Card) -> String {
    let (s, col) = match c.suit {
        Suit::Spade   => ("♥", "🟥"),
        Suit::Denari  => ("♦", "🟥"),
        Suit::Coppe   => ("♣", "⬛"),
        Suit::Bastoni => ("♠", "⬛"),
    };

    let num = match c.number {
        CardNum::Numeric(1)  => "A".to_string(),
        CardNum::Numeric(10) => "X".to_string(), // '10' is two characters, messes up the alignment
        CardNum::Numeric(n)  =>  n.to_string(),
        CardNum::Fante       => "J".to_string(),
        CardNum::Cavallo     => "Q".to_string(),
        CardNum::Re          => "K".to_string(),
    };

    format!("{}{}{}", s, num, col)  
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut s: String = String::new();
        s.push_str(&format!("Number of moves made is: \x1B[1m'{}'\x1B[0m\n\n", self.moves));
        s.push_str(&format!("\x1B[1mStack:\x1B[0m Top is {} ---- ({} cards upside down, {} passed)\n\n",
            self.stack.top().map(print_card_fr).unwrap_or("--".to_string()),
            self.stack.len(),
            self.passed_stack.len(),
            ));

        let print_ace = |i: usize| self.aces[i]
                                       .cards
                                       .iter()
                                       .last() // O(n) but prettier code :3
                                       .map(print_card_fr)
                                       .unwrap_or(UNKNOWN_CARD.to_string());

        s.push_str(&format!("          \t{}\t{}\t{}\t{}\n", 
                            "[0]",
                            "[1]",
                            "[2]",
                            "[3]"));

        s.push_str(&format!("\x1B[1mAce piles:\x1B[0m\t{}\t{}\t{}\t{}\n\n",
                            print_ace(0),
                            print_ace(1),
                            print_ace(2),
                            print_ace(3)));

        s.push_str("\x1B[1mMain area:\x1B[0m\n");

        let max_index: usize = self.piles.iter()
            .map(|p| p.cards.len()) // All lens
            .max().unwrap()         // Max len
            .max(1) - 1;            // Clamp to 1, turn into index

        s.push_str(&(0..7).map(|i| format!("[{i}]")).collect::<Vec<String>>().join("\t"));
        s.push('\n');
        s.push_str(&(0..7).map(|_| "===".to_string()).collect::<Vec<String>>().join("\t"));
        s.push('\n');

        let mut depth = 0;
        while depth <= max_index {
            for GamePile { cards, revealed } in &self.piles {
                if cards.get(depth).is_none() {
                    // Nothing
                } else if let (true, Some(card)) = (*revealed as usize >= cards.len()-depth,
                                                    cards.get(depth)) {
                    s.push_str(&print_card_fr(card));
                } else {
                    s.push_str(UNKNOWN_CARD);
                }

                s.push('\t');
            }
            depth += 1;
            s.push('\n');
        }
        s.push_str("\n\n");

        write!(f, "{s}")
    }
}

impl Default for Table {
    fn default() -> Self { Self::new() }
}


// ============ TESTS ================
#[test]
fn same_suit_mismatches() {
    let mut table = Table::new();
    let five = Card::new_fr(Suit::Coppe, 5);
    let six = Card::new_fr(Suit::Coppe, 6);

    // Correct number, wrong suit
    table.piles[0].cards[0] = five;
    table.piles[1].cards[1] = six;

    assert!(table.move_pile(0, 1, 1).is_err()); 

    assert_eq!(table.piles[0].cards[0], five); // Didn't get moved
    assert_eq!(table.piles[1].cards[1], six); // It didn't get changed
}

#[test]
fn diff_suit_mismatches() {
    let mut table = Table::new();
    let five = Card::new_fr(Suit::Bastoni, 5);
    let six = Card::new_fr(Suit::Coppe, 6);

    // Correct number, wrong suit
    table.piles[0].cards[0] = five;
    table.piles[1].cards[1] = six;

    assert!(table.move_pile(0, 1, 1).is_err()); 

    assert_eq!(table.piles[0].cards[0], five); // Didn't get moved
    assert_eq!(table.piles[1].cards[1], six); // It didn't get changed
}

#[test]
fn wrong_num_mismatches() {
    // We can try them all
    for n in 1..=13 { 
        if n == 5 { continue } // Don't want to test the correct one
        let mut table = Table::new();
        let ith = Card::new_fr(Suit::Denari, n);
        let six = Card::new_fr(Suit::Coppe, 6);

        // Correct suit, wrong number
        table.piles[0].cards[0] = ith;
        table.piles[1].cards[1] = six;

        assert!(table.move_pile(0, 1, 1).is_err()); 

        assert_eq!(table.piles[0].cards[0], ith); // Didn't get moved
        assert_eq!(table.piles[1].cards[1], six); // It didn't get changed
    }
}

#[test]
fn suit_match_legality() {
    let couples = [
        (Card::new_fr(Suit::Denari, 5), Card::new_fr(Suit::Coppe, 6)),
        (Card::new_fr(Suit::Coppe, 5), Card::new_fr(Suit::Denari, 6)),

        (Card::new_fr(Suit::Denari, 5), Card::new_fr(Suit::Bastoni, 6)),
        (Card::new_fr(Suit::Bastoni, 5), Card::new_fr(Suit::Denari, 6)),
        
        (Card::new_fr(Suit::Spade, 5), Card::new_fr(Suit::Coppe, 6)),
        (Card::new_fr(Suit::Coppe, 5), Card::new_fr(Suit::Spade, 6)),

        (Card::new_fr(Suit::Spade, 5), Card::new_fr(Suit::Bastoni, 6)),
        (Card::new_fr(Suit::Bastoni, 5), Card::new_fr(Suit::Spade, 6)),

    ];

    for (five, six) in couples {
        let mut table = Table::new();
        dbg!(&five, &six);
        table.piles[0].cards[0] = five;
        table.piles[1].cards[1] = six;

        assert!(table.move_pile(0, 1, 1).is_ok()); 

        assert!(table.piles[0].cards.is_empty()); // Didn't get moved
        assert_eq!(table.piles[1].cards[1], six); // It didn't get changed
        assert_eq!(table.piles[1].cards[2], five); // We gained a five
    }
}

#[test]
fn king_to_empty_pile() {
    let mut table = Table::new();
    table.piles = std::array::from_fn(|_i| Pile::default());
    table.piles[0].cards.push(Card::new_fr(Suit::Coppe, 13));
    table.piles[0].revealed = 1; // King is revealed

    table.move_pile(0, 1, 1); // Move King to empty pile

    assert!(table.piles[0].cards.is_empty()); // It got moved
    assert_eq!(table.piles[1].cards.get(0), Some(&Card::new_fr(Suit::Coppe, 13))); // It arrived
    
}
