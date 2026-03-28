use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub event_type: EventType,
    pub target_entry: Option<u32>,  // Which entry point, None = all
    pub threat_level: u32,          // 1-5
    pub reward: Option<EventReward>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    Raid,        // Goblin raid
    Ambush,      // Orc ambush
    Attack,      // Direct attack
    Dragon,      // Dragon sighting
    Nazgul,      // Nazgul terror
    Trade,       // Trade opportunity
    Festival,    // Celebration
    Discovery,   // Resource discovery
    Balrog,      // BOSS EVENT
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventReward {
    pub gold: u32,
    pub ore: u32,
    pub beer: u32,
    pub mithril: u32,
}

impl GameEvent {
    pub fn new(
        id: u32,
        name: &str,
        description: &str,
        event_type: EventType,
        target_entry: Option<u32>,
        threat_level: u32,
        reward: Option<EventReward>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            event_type,
            target_entry,
            threat_level,
            reward,
        }
    }
}

pub fn get_random_event(day: u32, darkness: f32) -> Option<GameEvent> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Day 300 = Balrog awakening (BOSS)
    if day >= 300 {
        return Some(GameEvent::new(
            100,
            "THE BALROG OF MORIA AWAKENS",
            "The ancient terror stirs. All entry points are under attack!",
            EventType::Balrog,
            None,
            5,
            None,
        ));
    }

    // Higher darkness = more dangerous events
    let base_chance = darkness as f32;

    if rng.gen::<f32>() > base_chance {
        return None; // No event this turn
    }

    // Select event based on day/darkness
    let event_id = rng.gen_range(1..=15);

    match event_id {
        1 => Some(GameEvent::new(
            1,
            "Goblin Raid",
            "A goblin raiding party approaches the mines!",
            EventType::Raid,
            Some(3), // Moria Mines
            2,
            None,
        )),
        2 => Some(GameEvent::new(
            2,
            "Merchant Caravan",
            "A friendly caravan brings trade goods!",
            EventType::Trade,
            Some(1), // Dale City Gates
            0,
            Some(EventReward { gold: 3, ore: 0, beer: 1, mithril: 0 }),
        )),
        3 if darkness > 0.3 => Some(GameEvent::new(
            3,
            "Orc Ambush",
            "Orcs launch a surprise attack at the mountain pass!",
            EventType::Ambush,
            Some(5), // Mountain Pass
            3,
            None,
        )),
        4 if darkness > 0.3 => Some(GameEvent::new(
            4,
            "Festival Day",
            "The dwarves hold a festival! Morale boosts!",
            EventType::Festival,
            Some(8), // Tavern Gate
            0,
            Some(EventReward { gold: 0, ore: 0, beer: 3, mithril: 0 }),
        )),
        5 if darkness > 0.4 => Some(GameEvent::new(
            5,
            "Warg Attack",
            "Warg riders strike the trade routes!",
            EventType::Attack,
            Some(6), // River Dock
            3,
            None,
        )),
        6 if darkness > 0.5 => Some(GameEvent::new(
            6,
            "Dragon Sighting",
            "A shadow passes overhead... is that Smaug?",
            EventType::Dragon,
            Some(2), // Treasury
            4,
            None,
        )),
        7 if darkness > 0.6 => Some(GameEvent::new(
            7,
            "Nazgul Terror",
            "The Nazgul fly over the kingdom! All defenders weakened!",
            EventType::Nazgul,
            None, // All
            4,
            None,
        )),
        8 => Some(GameEvent::new(
            8,
            "Ore Vein Discovered",
            "Miners find a rich mithril vein!",
            EventType::Discovery,
            Some(3), // Moria Mines
            0,
            Some(EventReward { gold: 0, ore: 4, beer: 0, mithril: 1 }),
        )),
        9 => Some(GameEvent::new(
            9,
            "Troll Emergence",
            "Stone trolls emerge from beneath the mountains!",
            EventType::Attack,
            Some(5), // Mountain Pass
            3,
            None,
        )),
        10 if darkness > 0.7 => Some(GameEvent::new(
            10,
            "Shadow Spreads",
            "Darkness creeps across the kingdom...",
            EventType::Attack,
            None, // All
            3,
            None,
        )),
        _ => Some(GameEvent::new(
            11,
            "Goblin Scouts",
            "Goblins scout the perimeter.",
            EventType::Raid,
            Some(rng.gen_range(1..=8)),
            1,
            None,
        )),
    }
}
