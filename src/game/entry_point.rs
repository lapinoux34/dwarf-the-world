// Backwards compatibility layer - maps old EntryPoint to new WorldZone
use serde::{Deserialize, Serialize};

pub use super::zone::{ZoneEffect, ZoneType, WorldZone, get_world_zones};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryType {
    Trade,
    Wealth,
    Resource,
    Supply,
    Military,
    Production,
    Recruitment,
    Any,
}

impl EntryType {
    pub fn synergy_bonus(&self) -> &'static str {
        match self {
            EntryType::Trade => "Trade",
            EntryType::Wealth => "Gold",
            EntryType::Resource => "Ore",
            EntryType::Supply => "Food",
            EntryType::Military => "Defense",
            EntryType::Production => "Upgrades",
            EntryType::Recruitment => "Heroes",
            EntryType::Any => "None",
        }
    }

    pub fn to_zone_type(&self) -> ZoneType {
        match self {
            EntryType::Trade => ZoneType::Trade,
            EntryType::Wealth => ZoneType::Wealth,
            EntryType::Resource => ZoneType::Resource,
            EntryType::Supply => ZoneType::Supply,
            EntryType::Military => ZoneType::Military,
            EntryType::Production => ZoneType::Production,
            EntryType::Recruitment => ZoneType::Recruitment,
            EntryType::Any => ZoneType::Any,
        }
    }
}

impl From<ZoneType> for EntryType {
    fn from(zt: ZoneType) -> Self {
        match zt {
            ZoneType::Trade => EntryType::Trade,
            ZoneType::Wealth => EntryType::Wealth,
            ZoneType::Resource => EntryType::Resource,
            ZoneType::Supply => EntryType::Supply,
            ZoneType::Military => EntryType::Military,
            ZoneType::Production => EntryType::Production,
            ZoneType::Recruitment => EntryType::Recruitment,
            ZoneType::Danger => EntryType::Any,
            ZoneType::Any => EntryType::Any,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceYield {
    pub gold: u32,
    pub ore: u32,
    pub beer: u32,
    pub food: u32,
    pub mithril: u32,
    pub runes: u32,
}

impl Default for ResourceYield {
    fn default() -> Self {
        Self {
            gold: 0,
            ore: 0,
            beer: 0,
            food: 0,
            mithril: 0,
            runes: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPoint {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub entry_type: EntryType,
    pub max_cards: u32,
    pub cards: Vec<u32>,
    pub defense_bonus: u32,
    pub resource_bonus: ResourceYield,
}

impl EntryPoint {
    pub fn from_zone(zone: &WorldZone) -> Self {
        let resource_bonus = match zone.zone_effect {
            ZoneEffect::GoldIncome(n) => ResourceYield { gold: n, ..Default::default() },
            ZoneEffect::GoldPerDwarf(n) => ResourceYield { gold: n, ..Default::default() },
            ZoneEffect::OreIncome(n) => ResourceYield { ore: n, ..Default::default() },
            ZoneEffect::BeerIncome(n) => ResourceYield { beer: n, ..Default::default() },
            _ => ResourceYield::default(),
        };

        let defense_bonus = match zone.zone_effect {
            ZoneEffect::DefenseBonus(n) => n,
            _ => 0,
        };

        Self {
            id: zone.id,
            name: zone.name.clone(),
            description: zone.description.clone(),
            entry_type: zone.zone_type.into(),
            max_cards: zone.max_cards,
            cards: zone.cards.clone(),
            defense_bonus,
            resource_bonus,
        }
    }

    pub fn new(
        id: u32,
        name: &str,
        description: &str,
        entry_type: EntryType,
        max_cards: u32,
    ) -> Self {
        let (defense_bonus, resource_bonus) = match entry_type {
            EntryType::Trade => (1, ResourceYield { gold: 1, ..Default::default() }),
            EntryType::Wealth => (0, ResourceYield { gold: 2, ..Default::default() }),
            EntryType::Resource => (0, ResourceYield { ore: 2, ..Default::default() }),
            EntryType::Supply => (0, ResourceYield { food: 1, beer: 1, ..Default::default() }),
            EntryType::Military => (3, ResourceYield::default()),
            EntryType::Production => (0, ResourceYield::default()),
            EntryType::Recruitment => (0, ResourceYield { beer: 1, ..Default::default() }),
            EntryType::Any => (0, ResourceYield::default()),
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

    pub fn can_play_card(&self, card_entry_type: EntryType) -> bool {
        card_entry_type == EntryType::Any || card_entry_type == self.entry_type
    }
}

// Legacy function for backwards compatibility
pub fn get_entry_points() -> Vec<EntryPoint> {
    let zones = get_world_zones();
    zones.iter().map(EntryPoint::from_zone).collect()
}
