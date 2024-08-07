use scopa::*;

fn main() {
    let mut game = Game::new();

    loop {
        let mut input = String::new();

        println!("Current player is: '{}'", game.color_playing());
        println!("Score is: {} '{}' - '{}' {}", purple_text(), game.purple_points, game.green_points, green_text());
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

        if let Some(tally) = game.is_match_over() {
            let (purp_p, gren_p) = match game.who_is_first {
                PlayerKind::Purple => (tally.first_points(), tally.shuf_points()),
                PlayerKind::Green  => (tally.shuf_points(), tally.first_points()),
            };
            println!("Match over: Purple got '{purp_p}' points, Green got '{gren_p}'");
            println!();
            println!("The breakdown is:\n{}\n", tally);
            game.purple_points += purp_p;
            game.green_points  += gren_p;

            // Full napoli takes preference over normal winner
            if has_full_napoli(&game.curr_match.player_first.pile) {
                println!("{} has achieved a full napoli: they win. What a nerd lmfao", game.who_is_first);
                break;
            }
            else if has_full_napoli(&game.curr_match.player_shuffler.pile) {
                println!("{} has achieved a full napoli: they win. What a nerd lmfao", !game.who_is_first);
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
            use std::{thread, time};

            clear_term();

            println!("Waiting 1.5 seconds before switching...");
            thread::sleep(time::Duration::from_millis(1500));
            game.toggle_turn();
        }
        clear_term();
    }
}

fn clear_term() {
    print!("{}[2J", 27 as char);
}
