use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    Dwarf,
    Monster,
    Event,
    Location,
    Resource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DwarfClass {
    Warrior,
    Miner,
    Brewer,
    Smith,
    Ranger,
    Mage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardEffect {
    Heal(u32),
    DrawCard,
    GainGold(u32),
    GainOre(u32),
    GainBeer(u32),
    GainRunes(u32),
    BuffNearby(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: u32,
    pub name: String,
    pub card_type: CardType,
    pub cost: u32,
    pub attack: Option<u32>,
    pub defense: Option<u32>,
    pub effect: Option<CardEffect>,
    pub art_path: String,
    pub dwarf_class: Option<DwarfClass>,
}

impl Card {
    pub fn new(
        id: u32,
        name: &str,
        card_type: CardType,
        cost: u32,
        attack: Option<u32>,
        defense: Option<u32>,
        effect: Option<CardEffect>,
        art_path: &str,
        dwarf_class: Option<DwarfClass>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            card_type,
            cost,
            attack,
            defense,
            effect,
            art_path: art_path.to_string(),
            dwarf_class,
        }
    }

    pub fn get_attack(&self) -> u32 {
        self.attack.unwrap_or(0)
    }

    pub fn get_defense(&self) -> u32 {
        self.defense.unwrap_or(0)
    }
}
