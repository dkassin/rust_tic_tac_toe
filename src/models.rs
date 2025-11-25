use serde::{Serialize, Deserialize};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    A,
    B,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub grid: [[Option<Player>; 3]; 3]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub board: Board,
    pub current_player: Player,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameStatus {
    InProgress,
    Won(Player),
    Draw,
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: [[None; 3]; 3],
        }
    }

    pub fn place_move(&mut self, row: usize, col: usize, player: Player) -> bool {
        // Check if position is valid and empty
        if row < 3 && col < 3 && self.grid[row][col].is_none() {
            self.grid[row][col] = Some(player);
            true // Move was successful
        } else {
            false // Invalid move
        }
    }

    pub fn check_winner(&self) -> Option<Player> {
        // check rows
        for row in 0..3 {
            if self.grid[row][0].is_some()
                && self.grid[row][0] == self.grid[row][1]
                && self.grid[row][1] == self.grid[row][2] {
                    return self.grid[row][0];
                }
        }
        // check columns
        for col in 0..3 {
            if self.grid[0][col].is_some()
                && self.grid[0][col] == self.grid[1][col]
                && self.grid[1][col] == self.grid[2][col] {
                    return self.grid[0][col];
                }
        }
        // check diagonal
        if self.grid[0][0].is_some()
            && self.grid[0][0] == self.grid[1][1]
            && self.grid[1][1] == self.grid[2][2] {
                return self.grid[0][0];
            }
        if self.grid[0][2].is_some()
            && self.grid[0][2] == self.grid[1][1]
            && self.grid[1][1] == self.grid[2][0] {
                return self.grid[0][2];
            }
        None
    }

    pub fn is_full(&self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.grid[row][col].is_none() {
                    return false;
                }
            }
        }
        true
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_player: Player::A,
        }
    }
    pub fn make_move(&mut self, row: usize, col: usize) -> bool {
        if self.board.place_move(row, col, self.current_player) {
            // Move was successful, switch players
            self.current_player = match self.current_player {
                Player::A => Player::B,
                Player::B => Player::A,
            };
            true
        } else {
            false
        }
    }
    pub fn get_status(&self) -> GameStatus {
        if let Some(winner) = self.board.check_winner() {
            GameStatus::Won(winner)
        } else if self.board.is_full() {
            GameStatus::Draw
        } else {
            GameStatus::InProgress
        }
    }
}