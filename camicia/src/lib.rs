use std::fmt::{Display, Debug, Formatter};
use std::collections::VecDeque;
use core::*;

#[derive(Clone, Debug)]
pub struct Game {
    pub turn: Turn,
    pub pile: Deck,
    pub player_first: Deck,
    pub player_second: Deck,
    /// Option of 1, 2 or 3 cards to force-throw
    pub forced_move: Option<usize>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Turn {
    #[default]
    First,
    Second,
}
    

impl Game {
    pub fn new() -> Game {
        let mut first = Card::shuffled_deck().0;
        let second = first.split_off(first.len()/2);
        Game {
            pile: Deck(VecDeque::new()),
            player_first: Deck(first),
            player_second: Deck(second),
            turn: Turn::First,
            forced_move: None,
        }
    }
    /// Option of winner
    pub fn is_over(&self) -> Option<Turn> {
        if self.player_first.is_empty() && self.turn == Turn::First {
            Some(Turn::Second)
        } else if self.player_second.is_empty() && self.turn == Turn::Second {
            Some(Turn::First)
        } else {
            None
        }
    }
    pub fn toggle_turn(&mut self) {
        self.turn = !self.turn;
    }
    pub fn tick(&mut self) {
        match (self.turn, self.forced_move) {
            (Turn::First, None) => {
                let c = self.player_first.take_from_top().expect("Ticked when player had no cards");
                if (1..=3).contains(&c.value()) {
                    self.forced_move = Some(c.value());
                }
                self.pile.push_to_top(c);
                self.turn = Turn::Second;
            },
            (Turn::Second, None) => {
                let c = self.player_second.take_from_top().expect("Ticked when player had no cards");
                if (1..=3).contains(&c.value()) {
                    self.forced_move = Some(c.value());
                }
                self.pile.push_to_top(c);
                self.turn = Turn::First;
            },
            (Turn::First, Some(forced_number)) => {
                let c = self.player_first.take_from_top().expect("Ticked when player had no cards");
                self.pile.push_to_top(c);
                if (1..=3).contains(&c.value()) {
                    self.forced_move = Some(c.value());
                    self.turn = Turn::Second;
                } else {
                    if forced_number == 1 {
                        self.pile.move_all_cards_to(&mut self.player_second);
                        self.forced_move = None;
                        self.turn = Turn::Second;
                    } else {
                        self.forced_move = Some(forced_number-1);
                    }
                }
            },
            (Turn::Second, Some(forced_number)) => {
                let c = self.player_second.take_from_top().expect("Ticked when player had no cards");
                self.pile.push_to_top(c);
                if (1..=3).contains(&c.value()) {
                    self.forced_move = Some(c.value());
                    self.turn = Turn::First;
                } else {
                    if forced_number == 1 {
                        self.pile.move_all_cards_to(&mut self.player_first);
                        self.forced_move = None;
                        self.turn = Turn::First;
                    } else {
                        self.forced_move = Some(forced_number-1);
                    }
                }
            },
        }
    }
}


impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f,
               r#"Turn: {:?}
First top card: {} ({} cards),
Second top card: {} ({} cards),
Pile top card: {} ({} cards),
Forced move?: {}
"#,
               self.turn,
               self.player_first.top().map(|c| c.to_string()).unwrap_or("NONE".to_string()),
               self.player_first.len(),
               self.player_second.top().map(|c| c.to_string()).unwrap_or("NONE".to_string()),
               self.player_second.len(),
               self.pile.top().map(|c| c.to_string()).unwrap_or("NONE".to_string()),
               self.pile.len(),
               self.forced_move.map(|n| n.to_string()).unwrap_or("NONE".to_string()),
        )
    }
}

impl std::ops::Not for Turn {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}

