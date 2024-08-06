use scopa::*;

fn main() {
    let mut game = Game::new();

    loop {
        let input = "whatever";
        match game.make_move(input) {
            Ok(()) => game.toggle_turn(),
            Err(e) => {
                println!("move error: {e:?}")
            }
        }

        if game.is_match_over() {
            // - Tally points
            // - Tell them them (TODO: Which points they got)
            // - Inc them
            // - Reset match

            game.
        }

        if let Some((player_name, win_p, lose_p)) = game.winner() {
            println!("{player_name} has won with {win_p} points! The loser had {lose_p} points")
        }
    }


    //println!("{}", game.curr_match);
}
