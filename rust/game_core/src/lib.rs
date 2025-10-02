use serde::{Deserialize, Serialize};
use rand_chacha::{ChaCha8Rng, rand_core::SeedableRng};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    pub value: u8,
}

impl Card {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub cards: Vec<Card>,
    #[serde(skip, default = "default_rng")]
    rng: ChaCha8Rng,
}

fn default_rng() -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(42)
}

impl Deck {
    pub fn new(seed: u64) -> Self {
        let mut cards = Vec::new();

        // Cards 1-12 have n copies each (card value 1 has 1 copy, card value 2 has 2 copies, etc.)
        for value in 1..=12 {
            for _ in 0..value {
                cards.push(Card::new(value));
            }
        }

        // One unique card with value 0
        cards.push(Card::new(0));

        let rng = ChaCha8Rng::seed_from_u64(seed);

        Self { cards, rng }
    }

    pub fn shuffle(&mut self) {
        use rand_chacha::rand_core::RngCore;

        // Fisher-Yates shuffle
        for i in (1..self.cards.len()).rev() {
            let j = (self.rng.next_u32() as usize) % (i + 1);
            self.cards.swap(i, j);
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn total_value(&self) -> u8 {
        self.cards.iter().map(|card| card.value).sum()
    }

    pub fn is_bust(&self) -> bool {
        self.total_value() > 21
    }

    pub fn has_flip7(&self) -> bool {
        // Flip7 is when hand contains cards that sum to exactly 7
        // This could be a single 7, or combinations like 3+4, 1+6, 2+5, 1+2+4, etc.
        let target = 7;
        let values: Vec<u8> = self.cards.iter().map(|card| card.value).collect();
        Self::can_sum_to_target(&values, target)
    }

    fn can_sum_to_target(values: &[u8], target: u8) -> bool {
        if target == 0 {
            return true;
        }
        if values.is_empty() || target > values.iter().sum::<u8>() {
            return false;
        }

        for (i, &value) in values.iter().enumerate() {
            if value == target {
                return true;
            }
            if value < target {
                let remaining = &values[i + 1..];
                if Self::can_sum_to_target(remaining, target - value) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub hand: Hand,
    pub score: u32,
    pub has_stayed: bool,
}

impl Player {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            hand: Hand::new(),
            score: 0,
            has_stayed: false,
        }
    }

    pub fn draw_card(&mut self, card: Card) {
        self.hand.add_card(card);
    }

    pub fn stay(&mut self) {
        self.has_stayed = true;
    }

    pub fn reset_for_round(&mut self) {
        self.hand = Hand::new();
        self.has_stayed = false;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundState {
    pub round_number: u32,
    pub current_player_index: usize,
    pub is_finished: bool,
}

impl RoundState {
    pub fn new() -> Self {
        Self {
            round_number: 1,
            current_player_index: 0,
            is_finished: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub round_state: RoundState,
}

impl GameState {
    pub fn new() -> Self {
        let deck = Deck::new(42); // Default seed
        Self {
            players: Vec::new(),
            deck,
            round_state: RoundState::new(),
        }
    }

    pub fn new_with_seed(seed: u64) -> Self {
        let deck = Deck::new(seed);
        Self {
            players: Vec::new(),
            deck,
            round_state: RoundState::new(),
        }
    }

    pub fn add_player(&mut self, id: String, name: String) {
        let player = Player::new(id, name);
        self.players.push(player);
    }

    pub fn start_round(&mut self) -> Result<(), String> {
        if self.players.is_empty() {
            return Err("No players added".to_string());
        }

        // Reset all players for new round
        for player in &mut self.players {
            player.reset_for_round();
        }

        // Create new deck and shuffle
        self.deck = Deck::new(42 + self.round_state.round_number as u64);
        self.deck.shuffle();

        // Deal initial cards (each player gets 2 cards)
        for _ in 0..2 {
            for player in &mut self.players {
                if let Some(card) = self.deck.draw() {
                    player.draw_card(card);
                }
            }
        }

        self.round_state.current_player_index = 0;
        self.round_state.is_finished = false;

        Ok(())
    }

    pub fn player_draw(&mut self, player_id: &str) -> Result<(), String> {
        if self.round_state.is_finished {
            return Err("Round is finished".to_string());
        }

        let current_player = &mut self.players[self.round_state.current_player_index];
        if current_player.id != player_id {
            return Err("Not your turn".to_string());
        }

        if current_player.has_stayed {
            return Err("Player has already stayed".to_string());
        }

        if let Some(card) = self.deck.draw() {
            current_player.draw_card(card);

            // Check if player is bust
            if current_player.hand.is_bust() {
                current_player.stay(); // Auto-stay on bust
            }

            // Move to next player
            self.advance_turn();
        } else {
            return Err("Deck is empty".to_string());
        }

        Ok(())
    }

    pub fn player_stay(&mut self, player_id: &str) -> Result<(), String> {
        if self.round_state.is_finished {
            return Err("Round is finished".to_string());
        }

        let current_player = &mut self.players[self.round_state.current_player_index];
        if current_player.id != player_id {
            return Err("Not your turn".to_string());
        }

        current_player.stay();
        self.advance_turn();

        Ok(())
    }

    fn advance_turn(&mut self) {
        self.round_state.current_player_index =
            (self.round_state.current_player_index + 1) % self.players.len();

        // Check if all players have stayed or busted
        if self.players.iter().all(|p| p.has_stayed) {
            self.round_state.is_finished = true;
        }
    }

    pub fn compute_scores(&mut self) -> HashMap<String, u32> {
        let mut scores = HashMap::new();

        for player in &mut self.players {
            let mut round_score = 0;

            if player.hand.has_flip7() {
                // Flip7 bonus
                round_score += 21;
            } else if !player.hand.is_bust() {
                // Normal scoring: hand value
                round_score += player.hand.total_value() as u32;
            }
            // Bust = 0 points

            player.score += round_score;
            scores.insert(player.id.clone(), round_score);
        }

        self.round_state.round_number += 1;
        scores
    }

    pub fn is_flip7(&self, player_id: &str) -> Result<bool, String> {
        let player = self.players.iter()
            .find(|p| p.id == player_id)
            .ok_or("Player not found")?;

        Ok(player.hand.has_flip7())
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_card_counts() {
        let deck = Deck::new(123);
        let mut card_counts = HashMap::new();

        for card in &deck.cards {
            *card_counts.entry(card.value).or_insert(0) += 1;
        }

        // Cards 1-12 should have n copies each
        for value in 1..=12 {
            assert_eq!(card_counts[&value], value as i32);
        }

        // Card 0 should have exactly 1 copy
        assert_eq!(card_counts[&0], 1);

        // Total should be 1+2+3+...+12+1 = 78+1 = 79
        assert_eq!(deck.cards.len(), 79);
    }

    #[test]
    fn test_bust_detection() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(10));
        hand.add_card(Card::new(12));

        assert!(hand.is_bust()); // 22 > 21

        let mut hand2 = Hand::new();
        hand2.add_card(Card::new(10));
        hand2.add_card(Card::new(11));

        assert!(!hand2.is_bust()); // 21 = 21
    }

    #[test]
    fn test_flip7_detection() {
        // Single 7 card
        let mut hand1 = Hand::new();
        hand1.add_card(Card::new(7));
        assert!(hand1.has_flip7());

        // Multiple cards summing to 7
        let mut hand2 = Hand::new();
        hand2.add_card(Card::new(3));
        hand2.add_card(Card::new(4));
        assert!(hand2.has_flip7());

        // Three cards summing to 7
        let mut hand3 = Hand::new();
        hand3.add_card(Card::new(1));
        hand3.add_card(Card::new(2));
        hand3.add_card(Card::new(4));
        assert!(hand3.has_flip7());

        // Cards not summing to 7
        let mut hand4 = Hand::new();
        hand4.add_card(Card::new(5));
        hand4.add_card(Card::new(6));
        assert!(!hand4.has_flip7());
    }

    #[test]
    fn test_scoring_accuracy() {
        let mut game = GameState::new();
        game.add_player("player1".to_string(), "Alice".to_string());
        game.add_player("player2".to_string(), "Bob".to_string());

        // Manually set up hands for testing
        game.players[0].hand.add_card(Card::new(7)); // Flip7
        game.players[1].hand.add_card(Card::new(10)); // Normal hand
        game.players[1].hand.add_card(Card::new(5)); // Total 15

        let scores = game.compute_scores();

        assert_eq!(scores["player1"], 21); // Flip7 bonus
        assert_eq!(scores["player2"], 15); // Hand value
    }

    #[test]
    fn test_game_flow() {
        let mut game = GameState::new();
        game.add_player("p1".to_string(), "Player 1".to_string());
        game.add_player("p2".to_string(), "Player 2".to_string());

        assert!(game.start_round().is_ok());

        // Each player should have 2 cards initially
        assert_eq!(game.players[0].hand.cards.len(), 2);
        assert_eq!(game.players[1].hand.cards.len(), 2);

        // Test serialization
        assert!(game.to_json().is_ok());
    }
}