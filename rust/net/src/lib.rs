use game_core::{GameState, GameMove, Player};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    JoinGame { player_name: String, game_id: Option<String> },
    StartGame { game_id: String },
    MakeMove { game_id: String, game_move: GameMove },
    GetGameState { game_id: String },
    LeaveGame { game_id: String, player_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    GameJoined { game_id: String, player_id: String },
    GameStarted { game_id: String },
    MoveAccepted { game_id: String },
    GameState { game_state: GameState },
    Error { message: String },
    PlayerLeft { game_id: String, player_id: String },
}

pub struct GameServer {
    games: Arc<RwLock<HashMap<String, GameState>>>,
}

impl GameServer {
    pub fn new() -> Self {
        Self {
            games: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn handle_message(&self, message: Message) -> Response {
        match message {
            Message::JoinGame { player_name, game_id } => {
                self.join_game(player_name, game_id).await
            }
            Message::StartGame { game_id } => {
                self.start_game(game_id).await
            }
            Message::MakeMove { game_id, game_move } => {
                self.make_move(game_id, game_move).await
            }
            Message::GetGameState { game_id } => {
                self.get_game_state(game_id).await
            }
            Message::LeaveGame { game_id, player_id } => {
                self.leave_game(game_id, player_id).await
            }
        }
    }

    async fn join_game(&self, player_name: String, game_id: Option<String>) -> Response {
        let mut games = self.games.write().await;

        let (game_id, game) = if let Some(id) = game_id {
            if let Some(game) = games.get_mut(&id) {
                (id, game)
            } else {
                return Response::Error {
                    message: "Game not found".to_string(),
                };
            }
        } else {
            let new_game = GameState::new(7, 7);
            let id = new_game.id.clone();
            games.insert(id.clone(), new_game);
            let game = games.get_mut(&id).unwrap();
            (id, game)
        };

        let player_id = game.add_player(player_name);

        Response::GameJoined {
            game_id: game_id.clone(),
            player_id,
        }
    }

    async fn start_game(&self, game_id: String) -> Response {
        let mut games = self.games.write().await;

        if let Some(game) = games.get_mut(&game_id) {
            match game.start_game() {
                Ok(()) => Response::GameStarted { game_id },
                Err(err) => Response::Error { message: err },
            }
        } else {
            Response::Error {
                message: "Game not found".to_string(),
            }
        }
    }

    async fn make_move(&self, game_id: String, game_move: GameMove) -> Response {
        let mut games = self.games.write().await;

        if let Some(game) = games.get_mut(&game_id) {
            match game.make_move(game_move) {
                Ok(()) => Response::MoveAccepted { game_id },
                Err(err) => Response::Error { message: err },
            }
        } else {
            Response::Error {
                message: "Game not found".to_string(),
            }
        }
    }

    async fn get_game_state(&self, game_id: String) -> Response {
        let games = self.games.read().await;

        if let Some(game) = games.get(&game_id) {
            Response::GameState {
                game_state: game.clone(),
            }
        } else {
            Response::Error {
                message: "Game not found".to_string(),
            }
        }
    }

    async fn leave_game(&self, game_id: String, player_id: String) -> Response {
        let mut games = self.games.write().await;

        if let Some(game) = games.get_mut(&game_id) {
            game.players.retain(|p| p.id != player_id);
            Response::PlayerLeft { game_id, player_id }
        } else {
            Response::Error {
                message: "Game not found".to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_join_new_game() {
        let server = GameServer::new();
        let response = server.handle_message(Message::JoinGame {
            player_name: "Alice".to_string(),
            game_id: None,
        }).await;

        match response {
            Response::GameJoined { game_id, player_id } => {
                assert!(!game_id.is_empty());
                assert!(!player_id.is_empty());
            }
            _ => panic!("Expected GameJoined response"),
        }
    }

    #[tokio::test]
    async fn test_start_game() {
        let server = GameServer::new();

        let join_response = server.handle_message(Message::JoinGame {
            player_name: "Alice".to_string(),
            game_id: None,
        }).await;

        let game_id = match join_response {
            Response::GameJoined { game_id, .. } => game_id,
            _ => panic!("Expected GameJoined response"),
        };

        server.handle_message(Message::JoinGame {
            player_name: "Bob".to_string(),
            game_id: Some(game_id.clone()),
        }).await;

        let start_response = server.handle_message(Message::StartGame {
            game_id: game_id.clone(),
        }).await;

        match start_response {
            Response::GameStarted { .. } => {}
            _ => panic!("Expected GameStarted response"),
        }
    }
}