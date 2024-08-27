use solitario::*;

fn main() {
    let table = Table::new();

    println!("Solving");

    println!("{:?}", solve_game(&table));
}
