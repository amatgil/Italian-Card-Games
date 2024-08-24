use solitario::*;


fn main() {
    let mut table = Table::new();
    for _ in 0..26 {
        println!("{table}");
        table.make_move("next");
    }
}
