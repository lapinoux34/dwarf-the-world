use serde::{Deserialize, Serialize};
use bevy::prelude::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZoneEffect {
    None,
    GoldIncome(u32),
    GoldPerDwarf(u32),
    OreIncome(u32),
    BeerIncome(u32),
    DefenseBonus(u32),
    AttackBonus(u32),
    DrawCardOnPlay,
    TradeBonus,
    DangerZone,
    FoodPrevention,
}

impl ZoneEffect {
    pub fn description(&self) -> String {
        match self {
            ZoneEffect::None => "No special effect".to_string(),
            ZoneEffect::GoldIncome(n) => format!("+{} gold per card played", n),
            ZoneEffect::GoldPerDwarf(n) => format!("+{} gold per dwarf stationed", n),
            ZoneEffect::OreIncome(n) => format!("+{} ore per card played", n),
            ZoneEffect::BeerIncome(n) => format!("+{} beer/turn if 3+ cards", n),
            ZoneEffect::DefenseBonus(n) => format!("+{} defense to all defenders", n),
            ZoneEffect::AttackBonus(n) => format!("+{} attack to dwarves crafted here", n),
            ZoneEffect::DrawCardOnPlay => "Draw +1 card when placing a card".to_string(),
            ZoneEffect::TradeBonus => "+1 gold per card played (trade bonus)".to_string(),
            ZoneEffect::DangerZone => "Monsters are stronger but drop more loot".to_string(),
            ZoneEffect::FoodPrevention => "Prevents beer shortage events".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZoneType {
    Trade,
    Wealth,
    Resource,
    Supply,
    Military,
    Production,
    Recruitment,
    Danger,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldZone {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub zone_type: ZoneType,
    pub effect: ZoneEffect,
    pub max_cards: u32,
    pub position: Vec2,
    pub cards: Vec<u32>,
    pub defense: u32,
    pub threatened: bool,
    pub threat_level: u32,
}

impl WorldZone {
    pub fn new(
        id: u32,
        name: &str,
        description: &str,
        zone_type: ZoneType,
        effect: ZoneEffect,
        max_cards: u32,
        position: Vec2,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            zone_type,
            effect,
            max_cards,
            position,
            cards: Vec::new(),
            defense: 0,
            threatened: false,
            threat_level: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.cards.len() >= self.max_cards as usize
    }

    pub fn add_card(&mut self, card_id: u32) -> bool {
        if self.is_full() {
            return false;
        }
        self.cards.push(card_id);
        true
    }
}

pub fn get_world_zones() -> Vec<WorldZone> {
    vec![
        WorldZone::new(
            1,
            "Dale City Gates",
            "Trade and commerce entry point",
            ZoneType::Trade,
            ZoneEffect::GoldIncome(1),
            5,
            Vec2::new(200.0, 300.0),
        ),
        WorldZone::new(
            2,
            "Erebor Treasury",
            "Riches and defense",
            ZoneType::Wealth,
            ZoneEffect::GoldPerDwarf(1),
            4,
            Vec2::new(400.0, 200.0),
        ),
        WorldZone::new(
            3,
            "Moria Mines",
            "Ore mining danger zone",
            ZoneType::Resource,
            ZoneEffect::OreIncome(1),
            6,
            Vec2::new(600.0, 350.0),
        ),
        WorldZone::new(
            4,
            "Dale Marketplace",
            "Food and allies supply",
            ZoneType::Supply,
            ZoneEffect::BeerIncome(1),
            4,
            Vec2::new(150.0, 450.0),
        ),
        WorldZone::new(
            5,
            "Mountain Pass",
            "Helm's Deep - military fortress",
            ZoneType::Military,
            ZoneEffect::DefenseBonus(2),
            5,
            Vec2::new(500.0, 400.0),
        ),
        WorldZone::new(
            6,
            "River Dock",
            "Long Lake - trade routes",
            ZoneType::Trade,
            ZoneEffect::DrawCardOnPlay,
            4,
            Vec2::new(300.0, 500.0),
        ),
        WorldZone::new(
            7,
            "Dwarven Forge",
            "Weapon crafting center",
            ZoneType::Production,
            ZoneEffect::AttackBonus(1),
            4,
            Vec2::new(550.0, 250.0),
        ),
        WorldZone::new(
            8,
            "Tavern Gate",
            "Mercenaries and heroes",
            ZoneType::Recruitment,
            ZoneEffect::None,
            3,
            Vec2::new(250.0, 350.0),
        ),
        WorldZone::new(
            9,
            "Mirkwood Border",
            "Dangerous wild zone",
            ZoneType::Danger,
            ZoneEffect::DangerZone,
            5,
            Vec2::new(700.0, 400.0),
        ),
        WorldZone::new(
            10,
            "Dale Farmlands",
            "Food supply reserves",
            ZoneType::Supply,
            ZoneEffect::FoodPrevention,
            4,
            Vec2::new(400.0, 550.0),
        ),
    ]
}
