use crate::game::{BoardLocation, LocationZone};

pub fn get_starter_locations() -> Vec<BoardLocation> {
    vec![
        BoardLocation::new(
            1,
            "Mine Entrance",
            "Where dwarves first emerge from the depths",
            5,
            LocationZone::MineEntrance,
        ),
        BoardLocation::new(
            2,
            "The Forge",
            "Weapons and armor crafting area",
            4,
            LocationZone::Forge,
        ),
        BoardLocation::new(
            3,
            "The Tavern",
            "Rest, beer, and merriment",
            4,
            LocationZone::Tavern,
        ),
        BoardLocation::new(
            4,
            "Mountain Peak",
            "High ground, ranger territory",
            3,
            LocationZone::MountainPeak,
        ),
        BoardLocation::new(
            5,
            "Underground Cavern",
            "Dark depths, dangerous creatures lurk within",
            5,
            LocationZone::UndergroundCavern,
        ),
    ]
}
