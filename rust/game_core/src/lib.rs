use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub id: String,
    pub players: Vec<Player>,
    pub board: Board,
    pub current_turn: usize,
    pub status: GameStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub width: u32,
    pub height: u32,
    pub tiles: HashMap<String, Tile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub value: Option<u32>,
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameStatus {
    Waiting,
    InProgress,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMove {
    pub player_id: String,
    pub x: u32,
    pub y: u32,
    pub value: u32,
}

impl GameState {
    pub fn new(width: u32, height: u32) -> Self {
        let mut tiles = HashMap::new();

        for x in 0..width {
            for y in 0..height {
                let key = format!("{}_{}", x, y);
                tiles.insert(key, Tile {
                    x,
                    y,
                    value: None,
                    owner: None,
                });
            }
        }

        Self {
            id: Uuid::new_v4().to_string(),
            players: Vec::new(),
            board: Board { width, height, tiles },
            current_turn: 0,
            status: GameStatus::Waiting,
        }
    }

    pub fn add_player(&mut self, name: String) -> String {
        let player_id = Uuid::new_v4().to_string();
        self.players.push(Player {
            id: player_id.clone(),
            name,
            score: 0,
        });
        player_id
    }

    pub fn make_move(&mut self, game_move: GameMove) -> Result<(), String> {
        if self.status != GameStatus::InProgress {
            return Err("Game is not in progress".to_string());
        }

        let current_player = &self.players[self.current_turn];
        if current_player.id != game_move.player_id {
            return Err("Not your turn".to_string());
        }

        let key = format!("{}_{}", game_move.x, game_move.y);
        let tile = self.board.tiles.get_mut(&key)
            .ok_or("Invalid position")?;

        if tile.value.is_some() {
            return Err("Position already occupied".to_string());
        }

        tile.value = Some(game_move.value);
        tile.owner = Some(game_move.player_id.clone());

        self.current_turn = (self.current_turn + 1) % self.players.len();

        Ok(())
    }

    pub fn start_game(&mut self) -> Result<(), String> {
        if self.players.len() < 2 {
            return Err("Need at least 2 players".to_string());
        }
        self.status = GameStatus::InProgress;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = GameState::new(7, 7);
        assert_eq!(game.board.width, 7);
        assert_eq!(game.board.height, 7);
        assert_eq!(game.board.tiles.len(), 49);
        assert_eq!(game.players.len(), 0);
    }

    #[test]
    fn test_add_player() {
        let mut game = GameState::new(7, 7);
        let player_id = game.add_player("Alice".to_string());
        assert_eq!(game.players.len(), 1);
        assert_eq!(game.players[0].name, "Alice");
        assert_eq!(game.players[0].id, player_id);
    }

    #[test]
    fn test_start_game() {
        let mut game = GameState::new(7, 7);
        game.add_player("Alice".to_string());
        game.add_player("Bob".to_string());

        assert!(game.start_game().is_ok());
        assert!(matches!(game.status, GameStatus::InProgress));
    }
}