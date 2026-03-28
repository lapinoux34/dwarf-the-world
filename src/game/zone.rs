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

impl ZoneType {
    pub fn synergy_bonus(&self) -> &'static str {
        match self {
            ZoneType::Trade => "Trade",
            ZoneType::Wealth => "Gold",
            ZoneType::Resource => "Ore",
            ZoneType::Supply => "Food/Beer",
            ZoneType::Military => "Defense",
            ZoneType::Production => "Upgrades",
            ZoneType::Recruitment => "Heroes",
            ZoneType::Danger => "Danger",
            ZoneType::Any => "None",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldZone {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub zone_type: ZoneType,
    pub location: Vec2,
    pub max_cards: u32,
    pub cards: Vec<u32>,
    pub defense: u32,
    pub attack_bonus: u32,
    pub defense_bonus: u32,
    pub zone_effect: ZoneEffect,
    pub threatened: bool,
    pub threat_level: u32,
    pub threatened_by: Option<u32>,
    pub synergies: Vec<SynergyBonus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynergyBonus {
    pub requirement: String,
    pub effect: String,
    pub active: bool,
}

impl WorldZone {
    pub fn new(
        id: u32,
        name: &str,
        description: &str,
        zone_type: ZoneType,
        location: Vec2,
        max_cards: u32,
        zone_effect: ZoneEffect,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            zone_type,
            location,
            max_cards,
            cards: Vec::new(),
            defense: 0,
            attack_bonus: 0,
            defense_bonus: 0,
            zone_effect,
            threatened: false,
            threat_level: 0,
            threatened_by: None,
            synergies: Self::calculate_synergies(zone_type),
        }
    }

    fn calculate_synergies(zone_type: ZoneType) -> Vec<SynergyBonus> {
        match zone_type {
            ZoneType::Resource => vec![
                SynergyBonus {
                    requirement: "3+ miners".to_string(),
                    effect: "+2 ore bonus".to_string(),
                    active: false,
                },
            ],
            ZoneType::Production => vec![
                SynergyBonus {
                    requirement: "2+ smiths".to_string(),
                    effect: "dwarves get +1 attack".to_string(),
                    active: false,
                },
            ],
            ZoneType::Supply => vec![
                SynergyBonus {
                    requirement: "Tavern + Marketplace".to_string(),
                    effect: "beer production doubled".to_string(),
                    active: false,
                },
            ],
            ZoneType::Military => vec![
                SynergyBonus {
                    requirement: "5+ defenders".to_string(),
                    effect: "fortress becomes impenetrable".to_string(),
                    active: false,
                },
            ],
            _ => vec![],
        }
    }

    pub fn is_full(&self) -> bool {
        self.cards.len() >= self.max_cards as usize
    }

    pub fn can_play_card(&self, card_zone_type: ZoneType) -> bool {
        card_zone_type == ZoneType::Any || card_zone_type == self.zone_type
    }

    pub fn add_card(&mut self, card_id: u32) {
        if !self.is_full() {
            self.cards.push(card_id);
        }
    }

    pub fn remove_card(&mut self, card_id: u32) {
        self.cards.retain(|&id| id != card_id);
    }

    pub fn set_threatened(&mut self, threatened: bool, threat_level: u32, threat_card: Option<u32>) {
        self.threatened = threatened;
        self.threat_level = threat_level;
        self.threatened_by = threat_card;
    }

    pub fn calculate_defense(&self, base_defense: u32) -> u32 {
        let synergy_bonus: u32 = self.synergies.iter()
            .filter(|s| s.active)
            .count() as u32 * 2;
        
        let effect_bonus = match self.zone_effect {
            ZoneEffect::DefenseBonus(n) => n,
            _ => 0,
        };
        
        base_defense + effect_bonus + synergy_bonus + self.defense_bonus
    }

    pub fn calculate_attack_bonus(&self, base_attack: u32) -> u32 {
        let effect_bonus = match self.zone_effect {
            ZoneEffect::AttackBonus(n) => n,
            _ => 0,
        };
        
        let synergy_bonus: u32 = self.synergies.iter()
            .filter(|s| s.active)
            .count() as u32;
        
        base_attack + effect_bonus + synergy_bonus
    }

    pub fn calculate_income(&self, card_count: u32) -> (u32, u32, u32, u32) {
        let mut gold = 0;
        let mut ore = 0;
        let mut beer = 0;
        let mut food = 0;

        match self.zone_effect {
            ZoneEffect::GoldIncome(n) => gold += n * card_count,
            ZoneEffect::GoldPerDwarf(n) => gold += n * card_count,
            ZoneEffect::OreIncome(n) => ore += n * card_count,
            ZoneEffect::BeerIncome(n) if card_count >= 3 => beer += n,
            ZoneEffect::TradeBonus => gold += card_count,
            _ => {}
        }

        (gold, ore, beer, food)
    }
}

pub fn get_world_zones() -> Vec<WorldZone> {
    vec![
        WorldZone::new(
            1,
            "Dale City Gates",
            "Trade and commerce. +1 gold per card played here per turn",
            ZoneType::Trade,
            Vec2::new(-350.0, 150.0),
            6,
            ZoneEffect::GoldIncome(1),
        ),
        WorldZone::new(
            2,
            "Erebor Treasury",
            "Riches. +1 gold per dwarf stationed here",
            ZoneType::Wealth,
            Vec2::new(-150.0, 150.0),
            5,
            ZoneEffect::GoldPerDwarf(1),
        ),
        WorldZone::new(
            3,
            "Moria Mines",
            "Ore mining. +1 ore per card played here per turn",
            ZoneType::Resource,
            Vec2::new(50.0, 150.0),
            6,
            ZoneEffect::OreIncome(1),
        ),
        WorldZone::new(
            4,
            "Dale Marketplace",
            "Food and allies. +1 beer per turn if you have 3+ cards here",
            ZoneType::Supply,
            Vec2::new(250.0, 150.0),
            5,
            ZoneEffect::BeerIncome(1),
        ),
        WorldZone::new(
            5,
            "Dale Farmlands",
            "Food supply. Prevents beer shortage events",
            ZoneType::Supply,
            Vec2::new(450.0, 150.0),
            4,
            ZoneEffect::FoodPrevention,
        ),
        WorldZone::new(
            6,
            "Mountain Pass",
            "Military defense. +2 defense to all dwarves here",
            ZoneType::Military,
            Vec2::new(-350.0, -50.0),
            6,
            ZoneEffect::DefenseBonus(2),
        ),
        WorldZone::new(
            7,
            "River Dock",
            "Trade routes. Draw +1 card when placing a card here",
            ZoneType::Trade,
            Vec2::new(-150.0, -50.0),
            4,
            ZoneEffect::DrawCardOnPlay,
        ),
        WorldZone::new(
            8,
            "Dwarven Forge",
            "Weapon crafting. +1 attack to dwarves crafted here",
            ZoneType::Production,
            Vec2::new(50.0, -50.0),
            5,
            ZoneEffect::AttackBonus(1),
        ),
        WorldZone::new(
            9,
            "Tavern Gate",
            "Mercenaries and heroes. Chance to get a free hero card",
            ZoneType::Recruitment,
            Vec2::new(250.0, -50.0),
            5,
            ZoneEffect::None,
        ),
        WorldZone::new(
            10,
            "Mirkwood Border",
            "Dangerous wild zone. Monsters here are stronger but drop more loot",
            ZoneType::Danger,
            Vec2::new(450.0, -50.0),
            5,
            ZoneEffect::DangerZone,
        ),
    ]
}
