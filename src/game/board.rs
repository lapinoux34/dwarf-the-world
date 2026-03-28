use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationZone {
    Erebor,     // The Lonely Mountain - treasure room
    Moria,      // Mines of Moria - cavern zones
    Dale,       // Dale Marketplace - trade zone
    HelmsDeep,  // Helm's Deep - defensive fortress
    Mirkwood,   // Mirkwood Forest - dangerous wild
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardLocation {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub max_cards: u32,
    pub zone: LocationZone,
}

impl BoardLocation {
    pub fn new(id: u32, name: &str, description: &str, max_cards: u32, zone: LocationZone) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            max_cards,
            zone,
        }
    }
}
