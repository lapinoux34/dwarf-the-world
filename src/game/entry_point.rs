// Compatibility layer - EntryPoint wraps WorldZone for backwards compatibility
use serde::{Deserialize, Serialize};
pub use super::zone::{WorldZone, ZoneEffect, ZoneType, get_world_zones};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceYield {
    pub gold: u32, pub ore: u32, pub beer: u32, pub food: u32, pub mithril: u32, pub runes: u32,
}

impl Default for ResourceYield {
    fn default() -> Self { Self { gold: 0, ore: 0, beer: 0, food: 0, mithril: 0, runes: 0 } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPoint {
    pub id: u32, pub name: String, pub description: String, pub entry_type: ZoneType,
    pub max_cards: u32, pub cards: Vec<u32>, pub defense_bonus: u32, pub resource_bonus: ResourceYield,
}

impl EntryPoint {
    pub fn from_zone(zone: &WorldZone) -> Self {
        let (defense_bonus, resource_bonus) = match zone.zone_effect {
            ZoneEffect::GoldIncome(n) => (0, ResourceYield { gold: n, ..Default::default() }),
            ZoneEffect::GoldPerDwarf(n) => (0, ResourceYield { gold: n, ..Default::default() }),
            ZoneEffect::OreIncome(n) => (0, ResourceYield { ore: n, ..Default::default() }),
            ZoneEffect::BeerIncome(n) => (0, ResourceYield { beer: n, ..Default::default() }),
            ZoneEffect::DefenseBonus(n) => (n, ResourceYield::default()),
            _ => (0, ResourceYield::default()),
        };
        Self { id: zone.id, name: zone.name.clone(), description: zone.description.clone(),
               entry_type: zone.zone_type, max_cards: zone.max_cards, cards: zone.cards.clone(),
               defense_bonus, resource_bonus }
    }

    pub fn is_full(&self) -> bool { self.cards.len() >= self.max_cards as usize }

    pub fn can_play_card(&self, card_zone_type: ZoneType) -> bool {
        card_zone_type == ZoneType::Any || card_zone_type == self.entry_type
    }
}

pub fn get_entry_points() -> Vec<EntryPoint> {
    let zones = get_world_zones();
    zones.iter().map(EntryPoint::from_zone).collect()
}
