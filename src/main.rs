extern crate dealer;

use dealer::structs::Score;
use dealer::results::get_score;
use dealer::results::declare_winner;
use dealer::new_game::create_game;

// TO DO:
// - What happens in the event of a split pot?
// - Print out the actual result of the contest
// - Think about how different the game will need to be to accomodate live play...
//   - API for betting
//   - API for returning status of game, current cards, revealing cards as they are dealt

fn main() {
    let mut player_1_winners = 0;
    let mut player_2_winners = 0;
    let mut split_pots = 0;

    for i in 0..20{
        let new_game = create_game();
        let player_1_score: Score = get_score(&new_game.player_1, &new_game.board);
        let player_2_score: Score = get_score(&new_game.player_2, &new_game.board);
        // player_1_score.print_score();
        // println!("FINAL SCORE: {}", player_1_score.final_score().0);
        // player_2_score.print_score();
        // println!("FINAL SCORE: {}", player_2_score.final_score().0);
        let winner = declare_winner(player_1_score, player_2_score);
        if winner.0.contains("Player 1 wins") {
            player_1_winners = player_1_winners + 1;
        }
        else if winner.0.contains("Player 2 wins") {
            player_2_winners = player_2_winners + 1;
        }
        else if winner.0.contains("split") {
            split_pots = split_pots + 1;
        }
    }
    println!("Player 1 wins: {}", player_1_winners);
    println!("Player 2 wins: {}", player_2_winners);
    println!("Split pots: {}", split_pots);
}
