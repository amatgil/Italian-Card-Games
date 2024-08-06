use scopa::*;

fn main() {
    let mut game = Game::new();

    loop {
        let input = "whatever";
        if let Err(e) = game.make_move(input) {
            println!("move error: {e:?}");
            continue;
        }

        if let Some((purp_p, gren_p)) = game.is_match_over() {
            println!("Match over: Purple got '{purp_p}' points, green got '{gren_p}'");
            println!("Starting new match....");
            game.purple_points += purp_p;
            game.green_points  += gren_p;
            game.curr_match = Match::new();
        }

        // TODO: compact this repeated code
        // if [ someone has llla kfsthsjhrekjghr gold] then win automaticaltnksjny
        if has_full_napoli(&game.curr_match.player_first.pile) {
            let winner = match game.curr_match.turn {
                Turn::First    => "Purple",
                Turn::Shuffler => "Green",
            };

            println!("{winner} has achieved a full napoli: they win. What a nerd lmfao");
            break;
        } else if has_full_napoli(&game.curr_match.player_shuffler.pile) {
            let winner = match game.curr_match.turn {
                Turn::First    => "Green",
                Turn::Shuffler => "Purple",
            };

            println!("{winner} has achieved a full napoli: they win. What a nerd lmfao");
            break;
        }

        if let Some((player_name, win_p, lose_p)) = game.winner() {
            println!("{player_name} has won with {win_p} points! The loser had {lose_p} points")
        }

        game.toggle_turn();
        println!("{}", game.curr_match);
    }


}
