use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};

#[derive(Row, Serialize, Deserialize)]
pub struct GameMoveRow {
    pub game_id: String,
    pub move_number: u32,
    pub board_state: String,
    pub current_player: String,
    pub game_status: String, 
}

pub trait GameRepository {
    fn insert_move(&mut self, row: GameMoveRow) -> Result<(), String>;
}

pub struct StubRepository {
    pub moves: Vec<GameMoveRow>,
}

impl StubRepository {
    pub fn new() -> Self {
        StubRepository { moves: Vec::new() }
    }
}

impl GameRepository for StubRepository {
    fn insert_move(&mut self, row: GameMoveRow) -> Result<(), String> {
        self.moves.push(row);
        Ok(())
    }
}

pub fn create_client() -> Client {
    Client::default()
        .with_url("http://localhost:8123")
        .with_database("tic_tac_toe")
        .with_user("default")
        .with_password("password123")
}

pub async fn insert_game_move(client: &Client, row: GameMoveRow) -> Result<(), clickhouse::error::Error> {
   let mut insert = client.insert("game_moves")?;
   insert.write(&row).await?;
   insert.end().await?;
   Ok(()) 
}