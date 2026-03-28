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
}
