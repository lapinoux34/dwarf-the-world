use rand::seq::SliceRandom;
use rand::thread_rng;

use super::card::Card;

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
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

    pub fn add_to_top(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_to_bottom(&mut self, card: Card) {
        self.cards.insert(0, card);
    }

    pub fn add_to_discard(&mut self, card: Card) {
        self.cards.push(card);
    }
}

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub max_size: usize,
}

impl Hand {
    pub fn new(max_size: usize) -> Self {
        Self {
            cards: Vec::new(),
            max_size,
        }
    }

    pub fn add_card(&mut self, card: Card) -> Option<Card> {
        if self.cards.len() >= self.max_size {
            return Some(card);
        }
        self.cards.push(card);
        None
    }

    pub fn remove_card(&mut self, card_id: u32) -> Option<Card> {
        if let Some(idx) = self.cards.iter().position(|c| c.id == card_id) {
            Some(self.cards.remove(idx))
        } else {
            None
        }
    }

    pub fn is_full(&self) -> bool {
        self.cards.len() >= self.max_size
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}
