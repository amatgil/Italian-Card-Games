use solitario::*;

use std::io;
use std::io::Write;

fn main() {
    let mut table = Table::new();
    let mut move_text_buffer = String::new();

    loop {
        println!("\n\n===========================================");
        println!("Current table is:\n{table}\n\n");

        println!("Please input your move:");
        print!(">");
        io::stdout().flush().expect("Could not flush stdout");

        io::stdin().read_line(&mut move_text_buffer).expect("Could not read from stdin");

        if let Err(e) = table.make_move(move_text_buffer.trim()) {
            println!("Error, go again; {e}");
        }

        move_text_buffer = String::new();
    }
}
