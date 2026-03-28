use serde::{Deserialize, Serialize};

use super::zone::WorldZone;
use super::card::Card;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub zones: Vec<WorldZone>,
}

impl Board {
    pub fn new(zones: Vec<WorldZone>) -> Self {
        Self { zones }
    }

    pub fn get_zone(&self, zone_id: u32) -> Option<&WorldZone> {
        self.zones.iter().find(|z| z.id == zone_id)
    }

    pub fn get_zone_mut(&mut self, zone_id: u32) -> Option<&mut WorldZone> {
        self.zones.iter_mut().find(|z| z.id == zone_id)
    }

    pub fn place_card(&mut self, zone_id: u32, card_id: u32) -> bool {
        if let Some(zone) = self.get_zone_mut(zone_id) {
            zone.add_card(card_id)
        } else {
            false
        }
    }

    pub fn remove_card(&mut self, zone_id: u32, card_id: u32) -> bool {
        if let Some(zone) = self.get_zone_mut(zone_id) {
            if let Some(idx) = zone.cards.iter().position(|&id| id == card_id) {
                zone.cards.remove(idx);
                return true;
            }
        }
        false
    }

    pub fn zone_card_count(&self, zone_id: u32) -> usize {
        self.get_zone(zone_id).map(|z| z.cards.len()).unwrap_or(0)
    }

    pub fn total_cards(&self) -> usize {
        self.zones.iter().map(|z| z.cards.len()).sum()
    }

    pub fn get_threatened_zones(&self) -> Vec<&WorldZone> {
        self.zones.iter().filter(|z| z.threatened).collect()
    }
}
