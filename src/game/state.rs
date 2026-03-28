use std::collections::HashMap;
use super::card::{Card, Resources, ResourceCost};
use super::entry_point::{EntryPoint, get_entry_points};
use super::event::GameEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Dawn,     // Collect resources
    Day,      // Play cards
    Dusk,     // Event revealed
    Night,    // Combat resolution
    EndTurn,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub day: u32,
    pub phase: Phase,
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub discard: Vec<Card>,
    pub entry_points: Vec<EntryPoint>,
    pub resources: Resources,
    pub selected_card: Option<Card>,
    pub selected_entry: Option<u32>,
    pub darkness_level: f32,
    pub current_event: Option<GameEvent>,
    pub player_hp: u32,
}

impl GameState {
    pub fn new(cards: Vec<Card>) -> Self {
        let entry_points = get_entry_points();

        Self {
            day: 1,
            phase: Phase::Dawn,
            deck: cards,
            hand: Vec::new(),
            discard: Vec::new(),
            entry_points,
            resources: Resources::new(),
            selected_card: None,
            selected_entry: None,
            darkness_level: 0.0,
            current_event: None,
            player_hp: 30,
        }
    }

    pub fn shuffle_deck(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
    }

    pub fn draw_cards(&mut self, count: usize) {
        for _ in 0..count {
            if let Some(card) = self.deck.pop() {
                if self.hand.len() < 10 {
                    self.hand.push(card);
                } else {
                    self.discard.push(card);
                }
            } else if !self.discard.is_empty() {
                // Shuffle discard into deck
                let mut discards = std::mem::take(&mut self.discard);
                self.deck.append(&mut discards);
                self.shuffle_deck();

                if let Some(card) = self.deck.pop() {
                    self.hand.push(card);
                }
            }
        }
    }

    pub fn play_card(&mut self, card: &Card, entry_id: u32) -> bool {
        // Check if entry exists and has space
        let entry = self.entry_points.iter_mut().find(|e| e.id == entry_id);
        if entry.is_none() || entry.map(|e| e.is_full()).unwrap_or(true) {
            return false;
        }

        // Check entry type compatibility
        let entry = entry.unwrap();
        if !entry.can_play_card(card.entry_type) {
            return false;
        }

        // Check resources
        if !self.resources.spend(&card.cost) {
            return false;
        }

        // Remove from hand
        if let Some(idx) = self.hand.iter().position(|c| c.id == card.id) {
            let played_card = self.hand.remove(idx);

            // Apply immediate effects
            if let Some(effect) = &played_card.effect {
                self.apply_effect(effect);
            }

            // Place in entry point
            if let Some(entry) = self.entry_points.iter_mut().find(|e| e.id == entry_id) {
                entry.cards.push(played_card.id);
            }

            self.selected_card = None;
            self.selected_entry = None;
            return true;
        }
        false
    }

    fn apply_effect(&mut self, effect: &super::card::CardEffect) {
        match effect {
            super::card::CardEffect::GenerateResource { resource, amount } => {
                match *resource {
                    "gold" => self.resources.gold += amount,
                    "ore" => self.resources.ore += amount,
                    "beer" => self.resources.beer += amount,
                    "food" => self.resources.food += amount,
                    "mithril" => self.resources.mithril += amount,
                    "runes" => self.resources.runes += amount,
                    _ => {}
                }
            }
            super::card::CardEffect::DrawCard => {
                self.draw_cards(1);
            }
            super::card::CardEffect::Heal { amount } => {
                self.player_hp = (self.player_hp + amount).min(30);
            }
            _ => {}
        }
    }

    pub fn collect_resources(&mut self) {
        // Base resources from entry points
        for entry in &self.entry_points {
            self.resources.gold += entry.resource_bonus.gold;
            self.resources.ore += entry.resource_bonus.ore;
            self.resources.beer += entry.resource_bonus.beer;
            self.resources.food += entry.resource_bonus.food;
            self.resources.mithril += entry.resource_bonus.mithril;
            self.resources.runes += entry.resource_bonus.runes;
        }

        // Passive effects from cards would be processed here
    }

    pub fn advance_day(&mut self) {
        self.day += 1;

        // Update darkness level (0.0 to 1.0 over 300 days)
        self.darkness_level = (self.day as f32 / 300.0).min(1.0);

        // Discard hand at end of day
        self.discard.extend(self.hand.drain(..));
        self.selected_card = None;
        self.selected_entry = None;
    }

    pub fn get_entry(&self, id: u32) -> Option<&EntryPoint> {
        self.entry_points.iter().find(|e| e.id == id)
    }

    pub fn get_entry_mut(&mut self, id: u32) -> Option<&mut EntryPoint> {
        self.entry_points.iter_mut().find(|e| e.id == id)
    }

    pub fn advance_phase(&mut self) {
        self.phase = match self.phase {
            Phase::Dawn => Phase::Day,
            Phase::Day => Phase::Dusk,
            Phase::Dusk => Phase::Night,
            Phase::Night => Phase::EndTurn,
            Phase::EndTurn => Phase::Dawn,
        };
    }
}
