use crate::game::{BoardLocation, LocationZone};

pub fn get_starter_locations() -> Vec<BoardLocation> {
    vec![
        BoardLocation::new(
            1,
            "The Lonely Mountain (Erebor)",
            "The great dwarf kingdom. Treasure vaults overflow with gold and mithril.",
            5,
            LocationZone::Erebor,
        ),
        BoardLocation::new(
            2,
            "Mines of Moria",
            "Dark caverns beneath the mountains. Rich in mithril, rich in danger.",
            5,
            LocationZone::Moria,
        ),
        BoardLocation::new(
            3,
            "Dale Marketplace",
            "Where dwarves and men trade goods, ale, and tales of adventure.",
            4,
            LocationZone::Dale,
        ),
        BoardLocation::new(
            4,
            "Helm's Deep",
            "The fortress of Rohan — allies defend together against the shadow.",
            4,
            LocationZone::HelmsDeep,
        ),
        BoardLocation::new(
            5,
            "Mirkwood Forest",
            "Twisted trees and spider-infested shadows. Tread carefully.",
            5,
            LocationZone::Mirkwood,
        ),
    ]
}
