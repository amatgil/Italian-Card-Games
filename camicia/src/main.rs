use camicia::*;

fn main() {
    let mut game = Game::new();

    let mut buffer = String::new();
    loop {
        //println!("{game}");
        //println!("Press the Any key for another move");
        //std::io::stdin().read_line(&mut buffer).expect("Could not read line from stdin");
        match game.is_over() {
            Some(winner) => {
                //println!("{winner:?} won! WOOO");
                break;
            },
            None => game.tick(),
        }
        let a = game.player_first.len();
        let b = game.player_second.len();
        let c = game.pile.len();
        println!("{} {} {}", a+b+c, b+a, c);
    }

}
