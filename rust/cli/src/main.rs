use clap::{Parser, Subcommand};
use game_core::GameState;
use std::fs;
use std::path::Path;

const GAME_STATE_FILE: &str = "game_state.json";

#[derive(Parser)]
#[command(name = "flip7_cli")]
#[command(about = "A CLI tool for debugging and testing Flip7 game scenarios")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new game
    New {
        /// Number of players
        #[arg(long, default_value = "2")]
        players: usize,
        /// Random seed for reproducible games
        #[arg(long, default_value = "42")]
        seed: u64,
    },
    /// Draw a card for a player
    Draw {
        /// Player ID (0-based index)
        player: usize,
    },
    /// Player chooses to stay
    Stay {
        /// Player ID (0-based index)
        player: usize,
    },
    /// Display current game state
    State,
    /// Simulate a series of commands from a script
    Simulate {
        /// Path to script file
        script: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { players, seed } => {
            if let Err(e) = handle_new(players, seed) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Draw { player } => {
            if let Err(e) = handle_draw(player) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Stay { player } => {
            if let Err(e) = handle_stay(player) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::State => {
            if let Err(e) = handle_state() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Simulate { script } => {
            if let Err(e) = handle_simulate(&script) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn handle_new(players: usize, seed: u64) -> Result<(), String> {
    if players < 1 {
        return Err("Number of players must be at least 1".to_string());
    }
    if players > 8 {
        return Err("Number of players cannot exceed 8".to_string());
    }

    let mut game = GameState::new_with_seed(seed);

    // Add players
    for i in 0..players {
        game.add_player(i.to_string(), format!("Player {}", i));
    }

    // Start the first round
    game.start_round().map_err(|e| format!("Failed to start round: {}", e))?;

    // Save game state
    save_game_state(&game)?;

    println!("New game started with {} players (seed: {})", players, seed);
    println!("Game state saved to {}", GAME_STATE_FILE);

    Ok(())
}

fn handle_draw(player: usize) -> Result<(), String> {
    let mut game = load_game_state()?;

    if player >= game.players.len() {
        return Err(format!("Player {} does not exist. Valid players: 0-{}", player, game.players.len() - 1));
    }

    let player_id = player.to_string();
    game.player_draw(&player_id).map_err(|e| format!("Draw failed: {}", e))?;

    save_game_state(&game)?;

    let player_obj = &game.players[player];
    println!("Player {} drew a card. Hand total: {} (cards: {})",
             player,
             player_obj.hand.total_value(),
             player_obj.hand.cards.len());

    if player_obj.hand.is_bust() {
        println!("Player {} is bust!", player);
    }
    if player_obj.hand.has_flip7() {
        println!("Player {} has Flip7!", player);
    }

    Ok(())
}

fn handle_stay(player: usize) -> Result<(), String> {
    let mut game = load_game_state()?;

    if player >= game.players.len() {
        return Err(format!("Player {} does not exist. Valid players: 0-{}", player, game.players.len() - 1));
    }

    let player_id = player.to_string();
    game.player_stay(&player_id).map_err(|e| format!("Stay failed: {}", e))?;

    save_game_state(&game)?;

    println!("Player {} stayed", player);

    // Check if round is finished
    if game.round_state.is_finished {
        println!("Round finished! Computing scores...");
        let scores = game.compute_scores();
        for (id, score) in scores {
            let player_idx: usize = id.parse().unwrap();
            println!("Player {}: {} points this round", player_idx, score);
        }
        save_game_state(&game)?;
    }

    Ok(())
}

fn handle_state() -> Result<(), String> {
    let game = load_game_state()?;
    let json = game.to_json().map_err(|e| format!("Failed to serialize game state: {}", e))?;
    println!("{}", json);
    Ok(())
}

fn handle_simulate(script_path: &str) -> Result<(), String> {
    if !Path::new(script_path).exists() {
        return Err(format!("Script file not found: {}", script_path));
    }

    let script_content = fs::read_to_string(script_path)
        .map_err(|e| format!("Failed to read script file: {}", e))?;

    for (line_num, line) in script_content.lines().enumerate() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        println!("Executing: {}", line);

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "new" => {
                let players = if parts.len() > 1 {
                    parts[1].parse().map_err(|_| format!("Invalid player count on line {}", line_num + 1))?
                } else { 2 };
                let seed = if parts.len() > 2 {
                    parts[2].parse().map_err(|_| format!("Invalid seed on line {}", line_num + 1))?
                } else { 42 };
                handle_new(players, seed)?;
            }
            "draw" => {
                if parts.len() < 2 {
                    return Err(format!("Missing player argument on line {}", line_num + 1));
                }
                let player = parts[1].parse()
                    .map_err(|_| format!("Invalid player ID on line {}", line_num + 1))?;
                handle_draw(player)?;
            }
            "stay" => {
                if parts.len() < 2 {
                    return Err(format!("Missing player argument on line {}", line_num + 1));
                }
                let player = parts[1].parse()
                    .map_err(|_| format!("Invalid player ID on line {}", line_num + 1))?;
                handle_stay(player)?;
            }
            "state" => {
                handle_state()?;
            }
            _ => {
                return Err(format!("Unknown command '{}' on line {}", parts[0], line_num + 1));
            }
        }
    }

    Ok(())
}

fn load_game_state() -> Result<GameState, String> {
    if !Path::new(GAME_STATE_FILE).exists() {
        return Err(format!("No game state found. Run 'cargo run -- new' to start a new game."));
    }

    let json = fs::read_to_string(GAME_STATE_FILE)
        .map_err(|e| format!("Failed to read game state: {}", e))?;

    GameState::from_json(&json)
        .map_err(|e| format!("Failed to parse game state: {}", e))
}

fn save_game_state(game: &GameState) -> Result<(), String> {
    let json = game.to_json()
        .map_err(|e| format!("Failed to serialize game state: {}", e))?;

    fs::write(GAME_STATE_FILE, json)
        .map_err(|e| format!("Failed to save game state: {}", e))?;

    Ok(())
}