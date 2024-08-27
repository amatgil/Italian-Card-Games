use camicia::*;

fn main() {
    let mut game = Game::new();
    let mut rounds = 0;

    const IS_AUTOMATIC: bool = true;  // Change at compile time

    let mut buffer = String::new();
    loop {
        if !IS_AUTOMATIC {
            println!("{game}");
            println!("Press the Any key for another move");
            std::io::stdin().read_line(&mut buffer).expect("Could not read line from stdin");
        }
        match game.is_over() {
            Some(winner) => {
                if !IS_AUTOMATIC { println!("{winner:?} won! WOOO. It took '{rounds}' rounds"); }
                break;
            },
            None => game.tick(),
        }

        if IS_AUTOMATIC {
            let a = game.player_first.len();
            let b = game.player_second.len();
            let c = game.pile.len();
            println!("{} {} {}", a+b+c, b+a, c);
        }

        rounds += 1;
    }

}
