use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub event_type: EventType,
    pub target_zone: Option<u32>,  // Which zone, None = all zones
    pub threat_level: u32,          // 0-5, scales with day
    pub reward: Option<EventReward>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    Raid,        // Minor attack
    Ambush,      // Surprise attack
    Attack,      // Direct attack
    Dragon,      // Dragon sighting
    Nazgul,      // Nazgul terror
    Trade,       // Trade opportunity (positive)
    Festival,    // Celebration (positive)
    Discovery,   // Resource discovery (positive)
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
        target_zone: Option<u32>,
        threat_level: u32,
        reward: Option<EventReward>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            event_type,
            target_zone,
            threat_level,
            reward,
        }
    }

    /// Calculate actual threat based on day number
    /// Days 1-30: GRACE PERIOD - no threats
    /// Days 31-100: Threats 1-2
    /// Days 101-200: Threats 2-3
    /// Days 201-299: Threats 3-4
    /// Day 300: Threat 5 (BOSS)
    pub fn scaled_threat(&self, day: u32) -> u32 {
        if day <= 30 {
            return 0; // No threats during grace period
        }
        let base = self.threat_level;
        let scaling = match day {
            1..=30 => 0,
            31..=100 => 0,     // Base threat only
            101..=200 => 1,    // +1 threat
            201..=299 => 2,    // +2 threat
            300 => 5,         // BOSS
            _ => 0,
        };
        (base + scaling).min(5)
    }

    /// Check if this event is positive (reward) or negative (threat)
    pub fn is_positive(&self) -> bool {
        matches!(
            self.event_type,
            EventType::Trade | EventType::Festival | EventType::Discovery
        )
    }
}

pub fn get_random_event(day: u32, darkness: f32) -> Option<GameEvent> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Day 300 = Balrog awakening (BOSS) - ALWAYS happens
    if day >= 300 {
        return Some(GameEvent::new(
            100,
            "THE BALROG OF MORIA AWAKENS",
            "The ancient terror stirs beneath the mountains. All zones are under attack!",
            EventType::Balrog,
            None, // Targets all zones
            5,
            None,
        ));
    }

    // Grace period: Days 1-30 - no threats, only opportunities
    if day <= 30 {
        let chance = rng.gen_range(1..=10);
        return match chance {
            1..=3 => Some(GameEvent::new(
                1,
                "Merchant Caravan",
                "A friendly caravan arrives with trade goods!",
                EventType::Trade,
                Some(1), // Dale City Gates
                0,
                Some(EventReward { gold: 3, ore: 0, beer: 1, mithril: 0 }),
            )),
            4..=6 => Some(GameEvent::new(
                2,
                "Ore Vein Discovery",
                "Miners discover a rich ore vein!",
                EventType::Discovery,
                Some(3), // Moria Mines
                0,
                Some(EventReward { gold: 0, ore: 3, beer: 0, mithril: 0 }),
            )),
            7..=8 => Some(GameEvent::new(
                3,
                "Festival Day",
                "The dwarves hold a celebration!",
                EventType::Festival,
                Some(9), // Tavern Gate
                0,
                Some(EventReward { gold: 1, ore: 0, beer: 2, mithril: 0 }),
            )),
            _ => None, // Sometimes nothing happens
        };
    }

    // Event chance increases with darkness
    // Early days: 30% chance, Late days: 70% chance
    let event_chance = 0.3 + (darkness * 0.4); // 0.3 to 0.7
    if rng.gen::<f32>() > event_chance {
        return None; // No event this turn
    }

    // Select event based on day and darkness
    // Higher darkness = more dangerous events
    let roll = rng.gen_range(1..=100);

    if day <= 100 {
        // Days 31-100: Mild threats
        match roll {
            1..=20 => Some(GameEvent::new(
                10,
                "Goblin Raid",
                "A goblin raiding party approaches the mines!",
                EventType::Raid,
                Some(3), // Moria Mines
                1,
                None,
            )),
            21..=35 => Some(GameEvent::new(
                11,
                "Merchant Caravan",
                "A friendly caravan brings trade goods!",
                EventType::Trade,
                Some(1), // Dale City Gates
                0,
                Some(EventReward { gold: 3, ore: 1, beer: 1, mithril: 0 }),
            )),
            36..=50 => Some(GameEvent::new(
                12,
                "Ore Vein Discovery",
                "Miners discover a rich ore vein!",
                EventType::Discovery,
                Some(3),
                0,
                Some(EventReward { gold: 0, ore: 4, beer: 0, mithril: 0 }),
            )),
            51..=65 => Some(GameEvent::new(
                13,
                "Goblin Scouts",
                "Goblins scout your perimeter.",
                EventType::Raid,
                Some(rng.gen_range(1..=10)),
                1,
                None,
            )),
            66..=75 => Some(GameEvent::new(
                14,
                "Festival Day",
                "The dwarves hold a celebration!",
                EventType::Festival,
                Some(9), // Tavern Gate
                0,
                Some(EventReward { gold: 1, ore: 0, beer: 3, mithril: 0 }),
            )),
            _ => Some(GameEvent::new(
                15,
                "Peaceful Day",
                "A quiet day in the kingdom.",
                EventType::Trade,
                None,
                0,
                Some(EventReward { gold: 1, ore: 0, beer: 0, mithril: 0 }),
            )),
        }
    } else if day <= 200 {
        // Days 101-200: Moderate threats
        match roll {
            1..=15 => Some(GameEvent::new(
                20,
                "Orc Ambush",
                "Orcs launch a surprise attack!",
                EventType::Ambush,
                Some(6), // Mountain Pass
                2,
                None,
            )),
            16..=28 => Some(GameEvent::new(
                21,
                "Warg Attack",
                "Warg riders strike the trade routes!",
                EventType::Attack,
                Some(7), // River Dock
                2,
                None,
            )),
            29..=40 => Some(GameEvent::new(
                22,
                "Troll Emergence",
                "Stone trolls emerge from beneath the mountains!",
                EventType::Attack,
                Some(6), // Mountain Pass
                2,
                None,
            )),
            41..=55 => Some(GameEvent::new(
                23,
                "Merchant Caravan",
                "A wealthy caravan offers good trade!",
                EventType::Trade,
                Some(1),
                0,
                Some(EventReward { gold: 4, ore: 2, beer: 1, mithril: 0 }),
            )),
            56..=68 => Some(GameEvent::new(
                24,
                "Festival Day",
                "The dwarves celebrate!",
                EventType::Festival,
                Some(9),
                0,
                Some(EventReward { gold: 1, ore: 0, beer: 3, mithril: 0 }),
            )),
            69..=78 => Some(GameEvent::new(
                25,
                "Ore Vein Discovery",
                "Miners find mithril!",
                EventType::Discovery,
                Some(3),
                0,
                Some(EventReward { gold: 0, ore: 3, beer: 0, mithril: 1 }),
            )),
            79..=88 => Some(GameEvent::new(
                26,
                "Goblin Raid",
                "Another goblin raid!",
                EventType::Raid,
                Some(rng.gen_range(1..=10)),
                1,
                None,
            )),
            _ => Some(GameEvent::new(
                27,
                "Shadow Spreads",
                "Darkness creeps closer...",
                EventType::Attack,
                Some(rng.gen_range(1..=10)),
                2,
                None,
            )),
        }
    } else {
        // Days 201-299: Severe threats
        match roll {
            1..=12 => Some(GameEvent::new(
                30,
                "Nazgul Terror",
                "The Nazgul fly over the kingdom! All defenders are weakened!",
                EventType::Nazgul,
                None, // All zones
                3,
                None,
            )),
            13..=25 => Some(GameEvent::new(
                31,
                "Dragon Sighting",
                "A dragon's shadow passes overhead...",
                EventType::Dragon,
                Some(2), // Treasury
                3,
                None,
            )),
            26..=38 => Some(GameEvent::new(
                32,
                "Orc Warband",
                "A large orc warband approaches!",
                EventType::Attack,
                Some(6), // Mountain Pass
                3,
                None,
            )),
            39..=50 => Some(GameEvent::new(
                33,
                "Shadow Army",
                "An army of darkness marches forth!",
                EventType::Attack,
                None, // All zones
                3,
                None,
            )),
            51..=62 => Some(GameEvent::new(
                34,
                "Troll Horde",
                "A horde of trolls emerges!",
                EventType::Attack,
                Some(3), // Moria Mines
                3,
                None,
            )),
            63..=75 => Some(GameEvent::new(
                35,
                "Last Merchant",
                "One final caravan offers supplies!",
                EventType::Trade,
                Some(7), // River Dock
                0,
                Some(EventReward { gold: 5, ore: 2, beer: 2, mithril: 0 }),
            )),
            76..=85 => Some(GameEvent::new(
                36,
                "Mithril Discovery",
                "Ancient dwarves found mithril!",
                EventType::Discovery,
                Some(8), // Dwarven Forge
                0,
                Some(EventReward { gold: 0, ore: 2, beer: 0, mithril: 2 }),
            )),
            _ => Some(GameEvent::new(
                37,
                "Desperate Defense",
                "Dark forces test your defenses...",
                EventType::Attack,
                Some(rng.gen_range(1..=10)),
                2,
                None,
            )),
        }
    }
}

/// Calculate threat level for combat based on day
pub fn get_threat_for_day(day: u32) -> u32 {
    match day {
        1..=30 => 0,      // Grace period
        31..=100 => 1,    // Mild
        101..=200 => 2,   // Moderate
        201..=299 => 3,   // Severe
        300 => 5,         // BOSS
        _ => 0,
    }
}
