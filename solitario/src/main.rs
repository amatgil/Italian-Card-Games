use solitario::*;
use cards_core::Card;

fn main() {
    let table = Table::new();

    println!("Solving");

    println!("Size of table is: {:?}B", std::mem::size_of::<Table>());
    println!("Size of card is: {:?}B", std::mem::size_of::<Card>());
    println!("Size of move is: {:?}B", std::mem::size_of::<ParsedMove>());
}
