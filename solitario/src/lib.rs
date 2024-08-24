use core::*;

mod parse;
use parse::*;

pub const UNKNOWN_CARD: &str = "--";

#[derive(Clone, Debug)]
pub struct Table {
    piles: [Pile; 7],
    stack: Deck,
    passed_stack: Deck,
    aces: [Pile; 4]
}

#[derive(Clone, Debug, Default)]
struct Pile {
    cards: Vec<Card>,
    revealed: usize,  // how many cards of this pile have been revealed
}


impl Pile {
    /// Get the card that's closest to the top of the table (as in, has the highest value)
    /// Assumes that `self.revealed > 0`
    fn get_head_of_revealed(&self) -> Option<&Card> {
        if self.cards.is_empty() { None }
        else {
            Some(&self.cards[self.cards.len() - self.revealed])
        }
    }
    /// Get the card that's closest to the bottom of the table (as in, has the lowest value)
    fn get_tail_of_revealed(&self) -> Option<&Card> {
        self.cards.iter().last() // O(n) but they won't get to 8, it's whatever
    }
}
impl Table {
    pub fn new() -> Self {
        let mut deck = Card::shuffled_french_deck();
        let mut piles = std::array::from_fn(|_i| Pile::default());
        for p in 0..7 {
            piles[p].revealed = 1;
            for _ in 0..p+1 {
                let card = deck.take_from_top().expect("Deck cannot be empty, we don't deal all cards");
                piles[p].cards.push(card);
            }
        }

        Self { piles,
               stack: deck,
               passed_stack: Deck::new(),
               aces: std::array::from_fn(|_i| Pile::default())
        }
    }
    pub fn make_move(&self, m: &str) -> Result<(), ()> {
        use ParsedMove as PM;
        match parse_move(m).map_err(|_| ())?.1 {
            PM::Undo => todo!("Undoing is not yet implemented"),
            PM::RevealNextOfStack => {
                todo!()
            },
            PM::MoveFromStackToPile(target) => {
                todo!()
            },
            PM::MoveFromStackToAce(target) => {
                todo!()
            },
            PM::MoveFromPileToPile { from, to, amount } => {
                todo!()
            },
            PM::MoveFromPileToAce { pile, ace } => {
                todo!()
            },
            PM::MoveFromAceToPile { ace, pile } => {
                todo!()
            }
        }
        Ok(())
    }
    pub fn move_pile(&mut self, from_idx: usize, to_idx: usize) -> Result<(), ()> {
        if from_idx >= 7 || to_idx >= 7 { return Err(()) }; // TODO: Make an error enum and whatever

        // We clone because we can't `&mut` them both at once, we'll reassign back if we're on the
        // happy path
        let mut from = self.piles[from_idx].clone();
        let mut to   = self.piles[to_idx].clone();

        let Some(from_head) = from.get_head_of_revealed() else { return Err(()) };
        let to_tail   = to.get_tail_of_revealed();


        
        if (to_tail.is_none() && from_head.number == CardNum::Re) // We're moving a King to empty
            || legality_check(from_head, to_tail.ok_or(())?) // Standard check
        {
            eprintln!("Legality/King check passed: {:?} and {:?}", from_head, to_tail);
            for _ in 0..from.revealed {
                dbg!(&from.cards);
                let c = from.cards.remove(from.revealed-1);
                to.cards.push(c);
            }


            to.revealed += from.revealed;
            from.revealed = 0;

            // We were on the happy path, we must reassign back
            self.piles[from_idx] = from;
            self.piles[to_idx] = to;
            Ok(())
        } else {
            Err(()) // Illegal move
        }
    }
}

// Denari and spade are red, coppe and bastoni are black. They must alternate
fn legality_check(added: &Card, base: &Card) -> bool {
    let red_suits = [Suit::Denari, Suit::Spade];
    let black_suits  = [Suit::Coppe, Suit::Bastoni];
    if (red_suits.contains(&base.suit) && red_suits.contains(&added.suit))
        || (black_suits.contains(&base.suit) && black_suits.contains(&added.suit)) {
        false
    } else { // Suits are fine, we check numbers
        added.value_fr() + 1 == base.value_fr()
    }
}

use std::fmt::Display;
use std::fmt::Formatter;
impl Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut s: String = String::new();
        s.push_str(&format!("Stack: Top is {} ---- ({} cards in it)\n\n",
            self.stack.top().map(|c| c.to_string()).unwrap_or("--".to_string()),
            self.stack.len()));

        let print_ace = |i: usize| self.aces[i].cards.iter()
                                                     .last() // O(n) but prettier code :3
                                                     .map(|c| c.to_string())
                                                     .unwrap_or(UNKNOWN_CARD.to_string());
        s.push_str(&format!("Ace piles (top cards):\t{}\t{}\t{}\t{}\n\n",
                            print_ace(0),
                            print_ace(1),
                            print_ace(2),
                            print_ace(3),
                            ));

        s.push_str(&format!("Main area:\n"));
        let max_index: usize = self.piles.iter()
            .map(|p| p.cards.len()) // All lens
            .max().unwrap()         // Max len
            .max(1) - 1;            // Clamp to 1, turn into index

        let mut depth = 0;
        while depth <= max_index {
            for Pile { cards, revealed } in &self.piles {
                if cards.get(depth).is_none() {
                    // Nothing
                } else if let (true, Some(card)) = (*revealed >= cards.len()-depth,
                                                    cards.get(depth)) {
                    s.push_str(&card.to_string());
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



// ============ TESTS ================
#[test]
fn same_suit_mismatches() {
    let mut table = Table::new();
    let five = Card::new_fr(Suit::Coppe, 5);
    let six = Card::new_fr(Suit::Coppe, 6);

    // Correct number, wrong suit
    table.piles[0].cards[0] = five;
    table.piles[1].cards[1] = six;

    assert!(table.move_pile(0, 1).is_err()); 

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

    assert!(table.move_pile(0, 1).is_err()); 

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

        assert!(table.move_pile(0, 1).is_err()); 

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

        assert!(table.move_pile(0, 1).is_ok()); 

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

    table.move_pile(0, 1); // Move King to empty pile

    assert!(table.piles[0].cards.is_empty()); // It got moved
    assert_eq!(table.piles[1].cards.get(0), Some(&Card::new_fr(Suit::Coppe, 13))); // It arrived
    
}
