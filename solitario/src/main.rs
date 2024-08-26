use solitario::*;

use std::io;
use std::io::Write;

const HOW_MANY_EQUALS: usize = 63;

fn main() {
    let mut table = Table::new();
    let mut move_text_buffer = String::new();

    loop {
        println!("\n\n{}\n", std::iter::repeat('=').take(HOW_MANY_EQUALS).collect::<String>());
        println!("{SYNTAX_CHEATSHEET}");
        println!("\n\n{}\n", std::iter::repeat('=').take(HOW_MANY_EQUALS).collect::<String>());

        println!("{}\n", std::iter::repeat('=').take(HOW_MANY_EQUALS).collect::<String>());
        println!("Current table is:\n{table}\n\n");
        println!("{}\n", std::iter::repeat('=').take(HOW_MANY_EQUALS).collect::<String>());

        println!("Please input your move:");
        print!(">");
        io::stdout().flush().expect("Could not flush stdout");

        io::stdin().read_line(&mut move_text_buffer).expect("Could not read from stdin");

        if let Err(e) = table.make_move(move_text_buffer.trim()) {
            println!("Error: \x1B[1;41m{e}\x1B[0m");
        }

        move_text_buffer = String::new();
    }
}
