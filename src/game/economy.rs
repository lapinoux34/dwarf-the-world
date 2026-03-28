use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Resources {
    pub gold: u32,
    pub ore: u32,
    pub beer: u32,
    pub runes: u32,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_gold(&mut self, amount: u32) {
        self.gold += amount;
    }

    pub fn add_ore(&mut self, amount: u32) {
        self.ore += amount;
    }

    pub fn add_beer(&mut self, amount: u32) {
        self.beer += amount;
    }

    pub fn add_runes(&mut self, amount: u32) {
        self.runes += amount;
    }

    pub fn spend_gold(&mut self, amount: u32) -> bool {
        if self.gold >= amount {
            self.gold -= amount;
            true
        } else {
            false
        }
    }
}
