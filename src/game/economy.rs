use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Resources {
    pub gold: u32,
    pub mithril: u32,
    pub provisions: u32,
    pub runestones: u32,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_gold(&mut self, amount: u32) {
        self.gold += amount;
    }

    pub fn add_mithril(&mut self, amount: u32) {
        self.mithril += amount;
    }

    pub fn add_provisions(&mut self, amount: u32) {
        self.provisions += amount;
    }

    pub fn add_runestones(&mut self, amount: u32) {
        self.runestones += amount;
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
