use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    Dwarf,
    Monster,
    Event,
    Location,
    Resource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DwarfFaction {
    Erebor,    // Thorin Company - treasure-focused
    Moria,     // Durin's Folk - mining, darker
    Dale,      // Allies, traders
    IronHills, // Warriors, defense
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DwarfClass {
    Leader,
    Warrior,
    Elder,
    Miner,
    Defender,
    Smith,
    RuneMaster,
    ShadowWalker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardEffect {
    Heal(u32),
    DrawCard,
    GainGold(u32),
    GainMithril(u32),
    GainProvisions(u32),
    GainRunestones(u32),
    BuffNearby(u32),
    WeakenEnemy(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: u32,
    pub name: String,
    pub card_type: CardType,
    pub cost: u32,
    pub attack: Option<u32>,
    pub defense: Option<u32>,
    pub effect: Option<CardEffect>,
    pub art_prompt: String,
    pub dwarf_class: Option<DwarfClass>,
    pub faction: Option<DwarfFaction>,
}

impl Card {
    pub fn new(
        id: u32,
        name: &str,
        card_type: CardType,
        cost: u32,
        attack: Option<u32>,
        defense: Option<u32>,
        effect: Option<CardEffect>,
        art_prompt: &str,
        dwarf_class: Option<DwarfClass>,
        faction: Option<DwarfFaction>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            card_type,
            cost,
            attack,
            defense,
            effect,
            art_prompt: art_prompt.to_string(),
            dwarf_class,
            faction,
        }
    }

    pub fn get_attack(&self) -> u32 {
        self.attack.unwrap_or(0)
    }

    pub fn get_defense(&self) -> u32 {
        self.defense.unwrap_or(0)
    }
}
