mod models;
use models::{Player, Board, Game, GameStatus};

fn main() {
    let mut game = Game::new();

    game.make_move(0, 0);
    game.make_move(1, 1);
    game.make_move(0, 1);

    println!("Game state: {:?}", game);
    println!("Game status: {:?}", game.get_status());
    println!("Winner: {:?}", game.board.check_winner());
}
