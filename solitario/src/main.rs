use solitario::*;


fn main() {
    let mut table = Table::new();
    println!("{table}");
    // Attempt move, hope it's legal (for testing)
    dbg!(table.move_pile(0, 1));
    println!("{table}");
}
