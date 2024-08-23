    use core::*;


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
                                                     .last()
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
