use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZoneType {
    Trade,
    Wealth,
    Resource,
    Supply,
    Military,
    Production,
    Recruitment,
    Any,
}

impl ZoneType {
    pub fn synergy_bonus(&self) -> &'static str {
        match self {
            ZoneType::Trade => "Trade",
            ZoneType::Wealth => "Gold",
            ZoneType::Resource => "Ore",
            ZoneType::Supply => "Food",
            ZoneType::Military => "Defense",
            ZoneType::Production => "Upgrades",
            ZoneType::Recruitment => "Heroes",
            ZoneType::Any => "None",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPoint {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub entry_type: ZoneType,
    pub max_cards: u32,
    pub cards: Vec<u32>,
    pub defense_bonus: u32,
    pub resource_bonus: ResourceYield,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceYield {
    pub gold: u32,
    pub ore: u32,
    pub beer: u32,
    pub food: u32,
    pub mithril: u32,
    pub runes: u32,
}

impl EntryPoint {
    pub fn new(
        id: u32,
        name: &str,
        description: &str,
        entry_type: ZoneType,
        max_cards: u32,
    ) -> Self {
        let (defense_bonus, resource_bonus) = match entry_type {
            ZoneType::Trade => (1, ResourceYield { gold: 1, ..Default::default() }),
            ZoneType::Wealth => (0, ResourceYield { gold: 2, ..Default::default() }),
            ZoneType::Resource => (0, ResourceYield { ore: 2, ..Default::default() }),
            ZoneType::Supply => (0, ResourceYield { food: 1, beer: 1, ..Default::default() }),
            ZoneType::Military => (3, ResourceYield::default()),
            ZoneType::Production => (0, ResourceYield::default()),
            ZoneType::Recruitment => (0, ResourceYield { beer: 1, ..Default::default() }),
            ZoneType::Any => (0, ResourceYield::default()),
        };

        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            entry_type,
            max_cards,
            cards: Vec::new(),
            defense_bonus,
            resource_bonus,
        }
    }

    pub fn is_full(&self) -> bool {
        self.cards.len() >= self.max_cards as usize
    }

    pub fn can_play_card(&self, card_entry_type: ZoneType) -> bool {
        card_entry_type == ZoneType::Any || card_entry_type == self.entry_type
    }
}

pub fn get_entry_points() -> Vec<EntryPoint> {
    vec![
        EntryPoint::new(1, "Dale City Gates", "Trade and commerce entry", ZoneType::Trade, 6),
        EntryPoint::new(2, "Erebor Treasury", "Riches and defense", ZoneType::Wealth, 5),
        EntryPoint::new(3, "Moria Mines", "Ore and danger", ZoneType::Resource, 6),
        EntryPoint::new(4, "Dale Marketplace", "Food and supplies", ZoneType::Supply, 5),
        EntryPoint::new(5, "Mountain Pass", "Military defense", ZoneType::Military, 6),
        EntryPoint::new(6, "River Dock", "River trade routes", ZoneType::Trade, 4),
        EntryPoint::new(7, "Dwarven Forge", "Weapon crafting", ZoneType::Production, 5),
        EntryPoint::new(8, "Tavern Gate", "Mercenaries and heroes", ZoneType::Recruitment, 5),
    ]
}
