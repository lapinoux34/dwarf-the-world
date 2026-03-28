use std::collections::HashMap;

use super::board::BoardLocation;
use super::card::Card;
use super::deck::Deck;
use super::economy::Resources;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Draw,
    Play,
    Combat,
    EndTurn,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub day: u32,
    pub phase: Phase,
    pub deck: Deck,
    pub hand: Vec<Card>,
    pub discard: Vec<Card>,
    pub board: HashMap<u32, Vec<Card>>,
    pub resources: Resources,
    pub selected_card: Option<Card>,
    pub selected_location: Option<u32>,
    pub player_hp: u32,
    pub combat_log: Vec<String>,
}

impl GameState {
    pub fn new(cards: Vec<Card>, locations: Vec<BoardLocation>) -> Self {
        let mut deck = Deck::new(cards);
        deck.shuffle();

        let mut board = HashMap::new();
        for loc in &locations {
            board.insert(loc.id, Vec::new());
        }

        Self {
            day: 1,
            phase: Phase::Draw,
            deck,
            hand: Vec::new(),
            discard: Vec::new(),
            board,
            resources: Resources::new(),
            selected_card: None,
            selected_location: None,
            player_hp: 30,
            combat_log: Vec::new(),
        }
    }

    pub fn draw_card(&mut self) {
        if let Some(card) = self.deck.draw() {
            if self.hand.len() < 10 {
                self.hand.push(card);
            } else {
                self.discard.push(card);
            }
        } else {
            // Deck empty - shuffle discard into deck
            if !self.discard.is_empty() {
                let discards: Vec<Card> = self.discard.drain(..).collect();
                self.deck = Deck::new(discards);
                self.deck.shuffle();
                if let Some(card) = self.deck.draw() {
                    self.hand.push(card);
                }
            }
        }
    }

    pub fn play_card(&mut self, card: &Card, location_id: u32) -> bool {
        // Remove from hand
        if let Some(idx) = self.hand.iter().position(|c| c.id == card.id) {
            let played = self.hand.remove(idx);

            // Add to board location
            if let Some(cards) = self.board.get_mut(&location_id) {
                cards.push(played);
                return true;
            }
        }
        false
    }

    pub fn end_turn(&mut self) {
        // Discard hand
        self.discard.extend(self.hand.drain(..));
        self.selected_card = None;
        self.selected_location = None;

        // Advance day
        self.day += 1;
        self.phase = Phase::Draw;
    }

    pub fn advance_phase(&mut self) {
        self.phase = match self.phase {
            Phase::Draw => Phase::Play,
            Phase::Play => Phase::Combat,
            Phase::Combat => Phase::EndTurn,
            Phase::EndTurn => Phase::Draw,
        };
    }
}
