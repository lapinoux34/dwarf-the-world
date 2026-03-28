use serde::{Deserialize, Serialize};
use super::entry_point::EntryType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    Settler,    // Establish presence, claim territory
    Builder,    // Construct buildings, structures
    Defender,   // Protect entry points
    Resource,   // Generate resources
    Hero,       // Powerful unique dwarves
    Ally,       // Non-dwarf helpers
    Spell,      // Magic effects
    Monster,    // Enemy cards to fight
    Event,      // Event cards
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DwarfFaction {
    Erebor,     // Thorin Company
    Moria,      // Durin's Folk
    Dale,       // Allies
    IronHills,  // Warriors
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct resourceCost {
    pub gold: u32,
    pub ore: u32,
    pub beer: u32,
    pub food: u32,
    pub mithril: u32,
    pub runes: u32,
}

impl resourceCost {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gold(amount: u32) -> Self {
        Self { gold: amount, ..Default::default() }
    }

    pub fn with_ore(mut self, amount: u32) -> Self {
        self.ore = amount;
        self
    }

    pub fn with_beer(mut self, amount: u32) -> Self {
        self.beer = amount;
        self
    }

    pub fn total(&self) -> u32 {
        self.gold + self.ore + self.beer + self.food + self.mithril + self.runes
    }

    pub fn can_afford(&self, resources: &Resources) -> bool {
        resources.gold >= self.gold
            && resources.ore >= self.ore
            && resources.beer >= self.beer
            && resources.food >= self.food
            && resources.mithril >= self.mithril
            && resources.runes >= self.runes
    }
}

impl std::fmt::Display for resourceCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}G {}O {}B {}F", self.gold, self.ore, self.beer, self.food)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Resources {
    pub gold: u32,
    pub ore: u32,
    pub beer: u32,
    pub food: u32,
    pub mithril: u32,
    pub runes: u32,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, other: &resourceCost) {
        self.gold += other.gold;
        self.ore += other.ore;
        self.beer += other.beer;
        self.food += other.food;
        self.mithril += other.mithril;
        self.runes += other.runes;
    }

    pub fn spend(&mut self, cost: &resourceCost) -> bool {
        if cost.can_afford(self) {
            self.gold -= cost.gold;
            self.ore -= cost.ore;
            self.beer -= cost.beer;
            self.food -= cost.food;
            self.mithril -= cost.mithril;
            self.runes -= cost.runes;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardEffect {
    GenerateResource { resource: String, amount: u32 },
    Defend { amount: u32 },
    Upgrade { amount: u32 },
    Heal { amount: u32 },
    DrawCard,
    BuffNearby { amount: u32 },
    WeakenEnemy { amount: u32 },
    StealResource { resource: String, amount: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: u32,
    pub name: String,
    pub card_type: CardType,
    pub cost: resourceCost,
    pub effect: Option<CardEffect>,
    pub entry_type: EntryType,
    pub art_prompt: String,
    pub faction: Option<DwarfFaction>,
    pub tier: u32,
    pub attack: Option<u32>,
    pub defense: Option<u32>,
}

impl Card {
    pub fn new(
        id: u32,
        name: &str,
        card_type: CardType,
        cost: resourceCost,
        effect: Option<CardEffect>,
        entry_type: EntryType,
        art_prompt: &str,
        faction: Option<DwarfFaction>,
        tier: u32,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            card_type,
            cost,
            effect,
            entry_type,
            art_prompt: art_prompt.to_string(),
            faction,
            tier,
            attack: None,
            defense: None,
        }
    }

    pub fn with_stats(mut self, attack: u32, defense: u32) -> Self {
        self.attack = Some(attack);
        self.defense = Some(defense);
        self
    }
}
