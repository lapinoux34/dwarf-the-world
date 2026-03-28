use crate::game::{Card, CardEffect, CardType, DwarfFaction, ZoneType, resourceCost};

pub fn get_starter_cards() -> Vec<Card> {
    vec![
        // === SETTLERS (Claim territory, basic presence) ===
        // Settlers are cheap but weak - they just claim territory
        Card::new(
            1,
            "Dwarf Settler",
            CardType::Settler,
            resourceCost::gold(2),
            Some(CardEffect::GenerateResource { resource: "gold".to_string(), amount: 1 }), // Weak but consistent
            ZoneType::Any,
            "Lord of the Rings dwarf settler pioneer detailed fantasy art",
            Some(DwarfFaction::Erebor),
            1,
        ),
        Card::new(
            2,
            "Pioneer Clan",
            CardType::Settler,
            resourceCost::gold(3).with_ore(1),
            Some(CardEffect::GenerateResource { resource: "ore".to_string(), amount: 1 }), // Same as Stone Mason but with settler type
            ZoneType::Resource,
            "Tolkien dwarf pioneer clan establishing settlement fantasy art",
            Some(DwarfFaction::Moria),
            2,
        ),
        Card::new(
            3,
            "Iron Hill Settler",
            CardType::Settler,
            resourceCost::gold(4),
            Some(CardEffect::Defend { amount: 2 }), // Military settler - can defend
            ZoneType::Military,
            "Iron Hills dwarf warrior settler detailed fantasy illustration",
            Some(DwarfFaction::IronHills),
            2,
        ),

        // === BUILDERS (Construct structures, generate resources) ===
        // Builders cost more but provide ongoing value
        Card::new(
            4,
            "Stone Mason",
            CardType::Builder,
            resourceCost::gold(2).with_ore(1),
            Some(CardEffect::GenerateResource { resource: "ore".to_string(), amount: 1 }), // 1 ore/turn - fair for 3 resource cost
            ZoneType::Production,
            "Dwarf stone mason builder Tolkien fantasy illustration",
            Some(DwarfFaction::Erebor),
            1,
        ),
        Card::new(
            5,
            "Master Architect",
            CardType::Builder,
            resourceCost::gold(5).with_ore(2),
            Some(CardEffect::BuffNearby { amount: 1 }), // +1 ATK to ALL dwarves in zone - strong but expensive
            ZoneType::Production,
            "Dwarf master architect grand design fantasy illustration",
            Some(DwarfFaction::Erebor),
            3,
        ),
        Card::new(
            6,
            "Mine Engineer",
            CardType::Builder,
            resourceCost::gold(3).with_ore(2),
            Some(CardEffect::GenerateResource { resource: "ore".to_string(), amount: 2 }), // 2 ore/turn - good value
            ZoneType::Resource,
            "Dwarf mine engineer mining machinery Tolkien fantasy",
            Some(DwarfFaction::Moria),
            2,
        ),

        // === DEFENDERS (Protect zones, absorb damage) ===
        // Defenders have high defense, some attack. Essential for survival.
        Card::new(
            7,
            "Militia",
            CardType::Defender,
            resourceCost::gold(2),
            None,
            ZoneType::Military,
            "Dwarf militia warrior basic soldier Tolkien fantasy art",
            None,
            1,
        ).with_stats(2, 2), // Balanced 1-cost defender
        Card::new(
            8,
            "Shield Bearer",
            CardType::Defender,
            resourceCost::gold(3).with_ore(1),
            Some(CardEffect::Defend { amount: 1 }), // +1 defense bonus
            ZoneType::Military,
            "Dwarf shield bearer defender Tolkien illustration",
            Some(DwarfFaction::IronHills),
            2,
        ).with_stats(2, 5), // High defense, low attack - tank
        Card::new(
            9,
            "Elite Guardian",
            CardType::Defender,
            resourceCost::gold(6),
            Some(CardEffect::Defend { amount: 2 }), // +2 defense bonus
            ZoneType::Wealth,
            "Dwarf elite guardian royal guard Tolkien fantasy art",
            Some(DwarfFaction::Erebor),
            3,
        ).with_stats(4, 8), // Elite defender - expensive but worth it

        // === RESOURCE CARDS (Generate resources each turn) ===
        // These are your economy engine. Cost = resource generation speed.
        // Rule of thumb: 3 resources cost should give ~2-3 resources/turn to be worth it
        Card::new(
            10,
            "Miner",
            CardType::Resource,
            resourceCost::gold(2), // Cost 2G, generates 1 ore/turn
            Some(CardEffect::GenerateResource { resource: "ore".to_string(), amount: 1 }), // Fair: 5 turns to pay off
            ZoneType::Resource,
            "Dwarf miner with pickaxe Tolkien fantasy illustration",
            Some(DwarfFaction::Moria),
            1,
        ),
        Card::new(
            11,
            "Goldsmith",
            CardType::Resource,
            resourceCost::gold(2).with_ore(1), // Cost 3G equivalent
            Some(CardEffect::GenerateResource { resource: "gold".to_string(), amount: 2 }), // 2 gold/turn - fair
            ZoneType::Wealth,
            "Dwarf goldsmith precious metals Tolkien fantasy art",
            Some(DwarfFaction::Erebor),
            2,
        ),
        Card::new(
            12,
            "Brewer",
            CardType::Resource,
            resourceCost::gold(2),
            Some(CardEffect::GenerateResource { resource: "beer".to_string(), amount: 1 }), // Beer economy
            ZoneType::Supply,
            "Dwarf brewer tavern keeper Tolkien fantasy illustration",
            Some(DwarfFaction::Erebor),
            1,
        ),
        Card::new(
            13,
            "Farmer",
            CardType::Resource,
            resourceCost::gold(2),
            Some(CardEffect::GenerateResource { resource: "food".to_string(), amount: 1 }), // Food economy
            ZoneType::Supply,
            "Dwarf farmer agriculture Tolkien fantasy art",
            Some(DwarfFaction::Dale),
            1,
        ),
        Card::new(
            14,
            "Rune Master",
            CardType::Resource,
            resourceCost::gold(3).with_ore(2), // Expensive - runes are powerful
            Some(CardEffect::GenerateResource { resource: "runes".to_string(), amount: 1 }),
            ZoneType::Recruitment,
            "Dwarf rune master ancient magic Tolkien illustration",
            Some(DwarfFaction::Moria),
            3,
        ),

        // === HEROES (Powerful unique dwarves) ===
        // Heroes are expensive but game-changing
        Card::new(
            15,
            "Thorin Oakenshield",
            CardType::Hero,
            resourceCost::gold(8),
            Some(CardEffect::BuffNearby { amount: 2 }), // +2 ATK to nearby dwarves
            ZoneType::Wealth,
            "Lord of the Rings Thorin Oakenshield king dwarf epic fantasy art",
            Some(DwarfFaction::Erebor),
            5,
        ).with_stats(6, 5), // Legendary stats
        Card::new(
            16,
            "Gimli",
            CardType::Hero,
            resourceCost::gold(6).with_beer(1),
            Some(CardEffect::BuffNearby { amount: 2 }), // +2 ATK to nearby dwarves
            ZoneType::Military,
            "Lord of the Rings Gimli warrior dwarf battle axe glowing runes",
            Some(DwarfFaction::Moria),
            4,
        ).with_stats(7, 5),
        Card::new(
            17,
            "Gloin",
            CardType::Hero,
            resourceCost::gold(5).with_ore(2),
            Some(CardEffect::GenerateResource { resource: "gold".to_string(), amount: 2 }), // Generates gold!
            ZoneType::Production,
            "Durin's Folk Gloin smith dwarf red beard fantasy art",
            Some(DwarfFaction::Moria),
            3,
        ).with_stats(4, 5),
        Card::new(
            18,
            "Balin",
            CardType::Hero,
            resourceCost::gold(4),
            Some(CardEffect::Heal { amount: 2 }), // Healing is valuable
            ZoneType::Any,
            "Tolkien Balin elder dwarf white beard detailed fantasy art",
            Some(DwarfFaction::Erebor),
            3,
        ).with_stats(3, 4),

        // === ALLIES (Non-dwarf helpers) ===
        Card::new(
            19,
            "Dale Man-at-Arms",
            CardType::Ally,
            resourceCost::gold(3),
            None,
            ZoneType::Trade,
            "Dale man warrior soldier Lord of the Rings fantasy illustration",
            Some(DwarfFaction::Dale),
            2,
        ).with_stats(3, 3), // Balanced ally
        Card::new(
            20,
            "Ranger of the North",
            CardType::Ally,
            resourceCost::gold(4).with_beer(1),
            Some(CardEffect::DrawCard), // Card draw is valuable
            ZoneType::Military,
            "Ranger warrior man Tolkien Lord of the Rings fantasy art",
            None,
            3,
        ).with_stats(4, 3),
        Card::new(
            21,
            "Elven Scout",
            CardType::Ally,
            resourceCost::gold(3),
            Some(CardEffect::StealResource { resource: "gold".to_string(), amount: 1 }), // Steals gold!
            ZoneType::Trade,
            "Elf scout woodland Lord of the Rings fantasy illustration",
            None,
            3,
        ).with_stats(3, 2),

        // === MONSTERS (Enemy threats - scale with day) ===
        // Monsters cost 0 but threaten zones. They appear via events, not played.
        // Stats scale with day number in combat resolution.
        Card::new(
            22,
            "Goblin Scout",
            CardType::Monster,
            resourceCost::gold(0),
            Some(CardEffect::StealResource { resource: "gold".to_string(), amount: 1 }),
            ZoneType::Any,
            "Moria goblin scout ugly dark creature fantasy illustration",
            None,
            1,
        ).with_stats(2, 1), // Day 1-50: threat 1
        Card::new(
            23,
            "Orc Warrior",
            CardType::Monster,
            resourceCost::gold(0),
            None,
            ZoneType::Military,
            "Orc warrior Sauron's army dark fantasy Tolkien illustration",
            None,
            2,
        ).with_stats(4, 3), // Day 31-100: threat 2
        Card::new(
            24,
            "Cave Troll",
            CardType::Monster,
            resourceCost::gold(0),
            Some(CardEffect::Defend { amount: 2 }), // Trolls are tanky
            ZoneType::Resource,
            "Tolkien cave troll stone giant dark fantasy illustration",
            None,
            3,
        ).with_stats(6, 5), // Day 51-150: threat 3
        Card::new(
            25,
            "Warg Rider",
            CardType::Monster,
            resourceCost::gold(0),
            None,
            ZoneType::Trade,
            "Warg rider orc mounted Tolkien dark fantasy illustration",
            None,
            3,
        ).with_stats(5, 2), // Fast attacker
        Card::new(
            26,
            "Orc Archer",
            CardType::Monster,
            resourceCost::gold(0),
            None,
            ZoneType::Military,
            "Orc archer dark bow Tolkien Lord of the Rings illustration",
            None,
            2,
        ).with_stats(3, 1), // Low defense but ranged
        Card::new(
            27,
            "Mirkwood Spider",
            CardType::Monster,
            resourceCost::gold(0),
            Some(CardEffect::WeakenEnemy { amount: 1 }), // Weakens defenders
            ZoneType::Supply,
            "Mirkwood giant spider Shelob descendant Tolkien dark fantasy",
            None,
            3,
        ).with_stats(4, 3),
        Card::new(
            28,
            "Dol Guldur Orc",
            CardType::Monster,
            resourceCost::gold(0),
            Some(CardEffect::BuffNearby { amount: 1 }), // Buffs nearby orcs
            ZoneType::Any,
            "Dol Guldur orc Sauron's servant dark fantasy illustration",
            None,
            2,
        ).with_stats(3, 2),
        Card::new(
            29,
            "Nazgul",
            CardType::Monster,
            resourceCost::gold(0),
            Some(CardEffect::WeakenEnemy { amount: 2 }), // Strong debuffer
            ZoneType::Any,
            "Nazgul wraith dark ringwraith Tolkien Lord of the Rings epic fantasy",
            None,
            4,
        ).with_stats(6, 4), // Day 201-299: threat 4
        Card::new(
            30,
            "THE BALROG OF MORIA",
            CardType::Monster,
            resourceCost::gold(0),
            Some(CardEffect::BuffNearby { amount: 3 }), // Buffs ALL monsters nearby
            ZoneType::Any,
            "Balrog demon of ancient Tolkien fire shadow Durin's Bane epic fantasy art",
            None,
            5,
        ).with_stats(12, 10), // BOSS: winnable with prep (15+ total defense needed)
    ]
}
