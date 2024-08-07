use scopa::*;

fn main() {
    let mut game = Game::new();

    loop {
        let mut input = String::new();

        println!("Current player is: '{}'", game.color_playing());
        println!("Score is:  Purple '{}' - '{}' Green", game.purple_points, game.green_points);
        println!("{}", game.curr_match);
        print!("You current cards are: ");
        game.print_cards_of_curr_player();


        println!("Waiting for input now....");
        std::io::stdin().read_line(&mut input).expect("Could not read from stdin");
        input = input.trim().to_string();

        if let Err(e) = game.make_move(&input) {
            clear_term();
            println!("move error: {e:?}");
            continue;
        }

        if let Some((purp_p, gren_p)) = game.is_match_over() {
            println!("Match over: Purple got '{purp_p}' points, Green got '{gren_p}'");
            game.purple_points += purp_p;
            game.green_points  += gren_p;

            // Full napoli takes preference over normal winner
            if has_full_napoli(&game.curr_match.player_first.pile) {
                println!("{} has achieved a full napoli: they win. What a nerd lmfao", game.whose_first);
                break;
            }
            else if has_full_napoli(&game.curr_match.player_shuffler.pile) {
                println!("{} has achieved a full napoli: they win. What a nerd lmfao", !game.whose_first);
                break;
            }
            else if let Some((player_name, win_p, lose_p)) = game.winner() {
                println!("{player_name} has won with {win_p} points! The loser had {lose_p} points, what a nerd lmao");
                break;
            }

            println!("Restarting match....");
            game.toggle_whose_first();
            game.curr_match = Match::new();

            println!("Press any button to start the next match...");
            std::io::stdin().read_line(&mut input).expect("Could not read from stdin");

            continue;
        } else {
            game.toggle_turn();
        }

        clear_term();

    }

}

fn clear_term() {
    print!("{}[2J", 27 as char);
}

        //// TODO: compact this repeated code
        //// if [ someone has llla kfsthsjhrekjghr gold] then win automaticaltnksjny
        //if has_full_napoli(&game.curr_match.player_first.pile) {
        //    let winner = match game.curr_match.turn {
        //        Turn::First    => "Purple",
        //        Turn::Shuffler => "Green",
        //    };

        //    println!("{winner} has achieved a full napoli: they win. What a nerd lmfao");
        //    break;
        //} else if has_full_napoli(&game.curr_match.player_shuffler.pile) {
        //    let winner = match game.curr_match.turn {
        //        Turn::First    => "Green",
        //        Turn::Shuffler => "Purple",
        //    };

        //    println!("{winner} has achieved a full napoli: they win. What a nerd lmfao");
        //    break;
        //}
