# Tic-Tac-Toe in Rust

A command-line tic-tac-toe game built in Rust with ClickHouse database integration. This project was created as a learning exercise to explore Rust fundamentals, serialization with Serde, and database integration patterns.

## Features

- Tic-tac-toe game logic with win/draw detection
- JSON serialization/deserialization using Serde
- Game state persistence to ClickHouse using ReplacingMergeTree
- Modular code organization
- Unit tests with trait-based stubbing

## Tech Stack

- **Rust** — Core language
- **Serde** — Serialization/deserialization
- **Tokio** — Async runtime
- **ClickHouse** — Database for game state history
- **Docker** — Local ClickHouse container

## Project Structure
```
src/
├── main.rs      # Entry point, game loop
├── models.rs    # Player, Board, Game, GameStatus
└── db.rs        # ClickHouse client, traits for testing
```

## Setup

### Prerequisites

- Rust (install via [rustup](https://rustup.rs/))
- Docker

### Start ClickHouse
```bash
docker run -d --name clickhouse-server \
  -p 8123:8123 \
  -p 9000:9000 \
  -e CLICKHOUSE_PASSWORD=password123 \
  clickhouse/clickhouse-server
```

### Create Database Table
```bash
docker exec -it clickhouse-server clickhouse-client --password password123
```
```sql
CREATE DATABASE IF NOT EXISTS tic_tac_toe;

USE tic_tac_toe;

CREATE TABLE game_moves (
    game_id String,
    move_number UInt32,
    board_state String,
    current_player String,
    game_status String,
    timestamp DateTime DEFAULT now()
) ENGINE = ReplacingMergeTree(move_number)
ORDER BY (game_id, move_number);
```

## Usage

### Run the Game
```bash
cargo run
```

### Run Tests
```bash
cargo test
```

### Query Game History
```sql
-- All moves
SELECT * FROM tic_tac_toe.game_moves;

-- Latest state per game (ReplacingMergeTree deduplication)
SELECT * FROM tic_tac_toe.game_moves FINAL;
```

## Concepts Covered

- Enums and structs
- `Option<T>` for nullable values
- Pattern matching with `match`
- Derive macros (`Debug`, `Clone`, `Copy`, `PartialEq`, `Serialize`, `Deserialize`)
- Module organization
- Async/await with Tokio
- Traits for dependency injection and testing
- Unit testing with `#[cfg(test)]`
