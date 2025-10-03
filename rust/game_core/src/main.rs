use game_core::*;

fn main() {
    println!("=== Flip7 Game Demo ===");

    // Initialize game state
    let mut game = GameState::new();
    println!("✓ GameState initialized successfully");

    // Add players
    game.add_player("player1".to_string(), "Alice".to_string());
    game.add_player("player2".to_string(), "Bob".to_string());
    println!("✓ Added 2 players: Alice and Bob");

    // Start round
    match game.start_round() {
        Ok(_) => println!("✓ Round started successfully"),
        Err(e) => {
            println!("✗ Failed to start round: {}", e);
            return;
        }
    }

    println!("\n=== Initial Hands ===");
    for player in game.players.iter() {
        println!(
            "{}: {} cards, total value: {}",
            player.name,
            player.hand.cards.len(),
            player.hand.total_value()
        );

        print!("  Cards: ");
        for card in &player.hand.cards {
            print!("{} ", card.value);
        }
        println!();

        if player.hand.has_flip7() {
            println!("  🎉 {} has FLIP7!", player.name);
        }
        if player.hand.is_bust() {
            println!("  💥 {} is BUST!", player.name);
        }
    }

    // Simulate some draws
    println!("\n=== Game Simulation ===");

    // Player 1 draws
    if let Err(e) = game.player_draw("player1") {
        println!("Player 1 draw failed: {}", e);
    } else {
        println!("Alice drew a card");
    }

    // Player 2 stays
    if let Err(e) = game.player_stay("player2") {
        println!("Player 2 stay failed: {}", e);
    } else {
        println!("Bob chose to stay");
    }

    // Player 1 stays (to end round)
    if let Err(e) = game.player_stay("player1") {
        println!("Player 1 stay failed: {}", e);
    } else {
        println!("Alice chose to stay");
    }

    println!("\n=== Final Results ===");
    let scores = game.compute_scores();

    for player in &game.players {
        println!(
            "{}: {} cards, total value: {}, round score: {}",
            player.name,
            player.hand.cards.len(),
            player.hand.total_value(),
            scores.get(&player.id).unwrap_or(&0)
        );

        if player.hand.has_flip7() {
            println!("  🎉 FLIP7 bonus!");
        }
        if player.hand.is_bust() {
            println!("  💥 BUST!");
        }
    }

    // Test serialization
    println!("\n=== Serialization Test ===");
    match game.to_json() {
        Ok(json) => {
            println!(
                "✓ GameState serialized successfully ({} characters)",
                json.len()
            );

            match GameState::from_json(&json) {
                Ok(_) => println!("✓ GameState deserialized successfully"),
                Err(e) => println!("✗ Deserialization failed: {}", e),
            }
        }
        Err(e) => println!("✗ Serialization failed: {}", e),
    }

    println!("\n=== Demo Complete ===");
}
