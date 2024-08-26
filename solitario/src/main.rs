use solitario::*;

use std::io;
use std::io::Write;

const HOW_MANY_EQUALS: usize = 63;

fn print_syntax_cheatsheet(equals_string: &str) {
    println!("\n\n{}\n", equals_string);
    println!("{SYNTAX_CHEATSHEET}");
    println!("\n\n{}\n", equals_string);
}

fn main() {
    let mut table = Table::new();
    let mut move_text_buffer = String::new();

    let equals_string = "=".repeat(HOW_MANY_EQUALS);

    print_syntax_cheatsheet(&equals_string);

    loop {
        //print!("\x1B[2J"); // this make the errors not show up lmao
        io::stdout().flush().expect("Could not flush stdout");

        print_syntax_cheatsheet(&equals_string);

        println!("{}\n", equals_string);
        println!("Current table is:\n{table}\n\n");
        println!("{}\n", equals_string);

        println!("Please input your move:");
        print!(">");
        io::stdout().flush().expect("Could not flush stdout");

        io::stdin().read_line(&mut move_text_buffer).expect("Could not read from stdin");


        if let Err(e) = table.make_move(move_text_buffer.trim()) {
            println!("Error: \x1B[1;41m{e}\x1B[0m");
        }

        if table.has_won() {

            println!(
r#"__   __          _                                 _ 
\ \ / /__  _   _( )_   _____  __      _____  _ __ | |
 \ V / _ \| | | |/\ \ / / _ \ \ \ /\ / / _ \| '_ \| |
  | | (_) | |_| |  \ V /  __/  \ V  V / (_) | | | |_|
  |_|\___/ \__,_|   \_/ \___|   \_/\_/ \___/|_| |_(_)
                                                     
__        __                            
\ \      / /__   ___   ___   ___   ___  
 \ \ /\ / / _ \ / _ \ / _ \ / _ \ / _ \ 
  \ V  V / (_) | (_) | (_) | (_) | (_) |
   \_/\_/ \___/ \___/ \___/ \___/ \___/ 
                                        
"#);
            break;
        }

        move_text_buffer = String::new();
    }
}
