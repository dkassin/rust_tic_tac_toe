mod models;
mod db;
use models::{Game, GameStatus};
use db::{create_client, insert_game_move, GameMoveRow};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let client = create_client();
    let game_id = Uuid::new_v4().to_string();

    let mut game = Game::new();
    let mut move_number: u32 = 0;

    println!("Starting game: {}", game_id);

    // Simulate a game - A wins with top row
    let moves = vec![(0,0),(1,0),(0,1),(1,1),(0,2)];

    for (row,col) in moves {
        let current = format!("{:?}", game.current_player);
        game.make_move(row, col);
        move_number += 1;

        let row_data = GameMoveRow {
            game_id: game_id.clone(),
            move_number,
            board_state: serde_json::to_string(&game.board).unwrap(),
            current_player: format!("{:?}", game.current_player),
            game_status: format!("{:?}", game.get_status()),
        };

        match insert_game_move(&client, row_data).await {
            Ok(_) => println!("Move {}: Player {} at ({},{})", move_number, current, row, col),
            Err(e) => println!("Error inserting move: {}", e),
        }
    }
    
    println!("\nFinal board: {:?}", game.board);
    println!("Game status: {:?}", game.get_status())
}
