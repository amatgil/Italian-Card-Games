use scopa::*;

fn main() {
    let deck = Card::shuffled_deck();
    for card in deck {
        println!("{card}");
    }
}
