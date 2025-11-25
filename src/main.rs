mod models;
use models::{Game, GameStatus};

fn main() {
    let mut game = Game::new();

    game.make_move(0, 0);
    game.make_move(1, 1);
    game.make_move(0, 1);

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&game).unwrap();
    println!("Game as JSON:\n{}", json);

    let restored_game: Game = serde_json::from_str(&json).unwrap();
    println!("\nRestored game: {:?}", restored_game);
}
