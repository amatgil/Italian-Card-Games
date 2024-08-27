use solitario::*;

fn main() {
    let table = Table::new();

    println!("Solving");

    println!("Size of table is: {:?}B", std::mem::size_of::<Table>());
    println!("Size of move is: {:?}B", std::mem::size_of::<ParsedMove>());
}
