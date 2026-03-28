# Dwarf the World — Kingdom Conquest Game

## 1. Project Overview

**Type:** World/Region Control Game with Deck-Building Mechanics
**Engine:** Rust + Bevy
**Theme:** Tolkien / Lord of the Rings Dwarf Kingdom Management
**Inspiration:** Slay the Spire meets Civilization meets Dwarf Fortress world map
**Goal:** Survive 300 days by managing your dwarf kingdom across multiple strategic entry points

You control a **Dwarf Kingdom/Region** with **8+ strategic entry points** on the world map. Deploy cards to expand territory, gather resources, defend against threats, and survive until Day 300 when the Balrog awakens.

---

## 2. Visual Design

### Color Palette
- **Primary:** Gold (#D4AF37), Bronze (#CD7F32)
- **Backgrounds:** Deep brown (#2D1B0E), Cave darkness (#0D0A05), Charcoal (#1A1410)
- **Accent:** Torch orange (#FF6B00), Ember red (#8B0000)
- **UI Borders:** Mithril silver (#C0C0C0), Dwarven bronze (#B87333)

### Typography
- Kingdom titles: Bold fantasy serif
- Stats: Clean, readable pixel-style

### Card Art Style
Gritty fantasy realism. AI prompts: "Lord of the Rings dwarf warrior detailed fantasy art", "Moria goblin dark fantasy illustration"

---

## 3. World Map & Entry Points

The world map shows your **Dwarf Kingdom** with **8 strategic entry points**. Each entry point is a zone where you play cards for different strategic purposes.

### Entry Points

| # | Name | Type | Purpose | Max Cards |
|---|------|------|---------|-----------|
| 1 | **Dale City Gates** | Trade | Commerce, diplomacy, allies | 6 |
| 2 | **Erebor Treasury** | Wealth | Gold generation, defense | 5 |
| 3 | **Moria Mines** | Resource | Ore mining, mithril gathering | 6 |
| 4 | **Dale Marketplace** | Supply | Food, beer, provisions | 5 |
| 5 | **Mountain Pass** | Military | Defense, combat, wall building | 6 |
| 6 | **River Dock** | Trade | River trade,外来 allies | 4 |
| 7 | **Dwarven Forge** | Production | Weapon crafting, upgrades | 5 |
| 8 | **Tavern Gate** | Recruitment | Mercenaries, heroes, quests | 5 |

### Entry Point Mechanics
- Each entry point has a **type** (Trade, Military, Resource, etc.)
- **Card synergies** activate based on entry point type
- Cards placed generate **type-specific bonuses**
- Entry points can be **upgraded** by placing matching cards
- Enemies attack specific entry points during events

---

## 4. Card System

### Card Types

| Type | Purpose | Examples |
|------|---------|----------|
| **Settler** | Establish presence, claim territory | Dwarf settler, Pioneer clan |
| **Builder** | Construct buildings, structures | Stone mason, Architect |
| **Defender** | Protect entry points, walls | Warrior, Archer, Wall |
| **Resource** | Generate gold, ore, food, beer | Miner, Farmer, Merchant |
| **Hero** | Powerful unique dwarves | Gimli, Thorin, Gloin |
| **Ally** | Non-dwarf helpers | Dale men, Rangers, Elves |
| **Spell** | Magic effects, buffs | Rune magic, Blessing |
| **Monster** | Enemy cards to fight | Goblin, Orc, Troll |

### Card Structure
```
- id: u32
- name: String
- card_type: CardType
- cost: Resources          # What it costs to play
- effect: CardEffect        # What it does
- entry_type: EntryType    # Best entry point type
- art_prompt: String        # AI art generation
- faction: Option<DwarfFaction>
- tier: u32                 # 1-5 (common to legendary)
```

### Sample Cards

**SETTLERS:**
| Name | Cost | Effect | Entry Type |
|------|------|--------|------------|
| Dwarf Settler | 2 Gold | Place settler marker | Any |
| Pioneer Clan | 3 Gold, 1 Ore | Settle + gain 1 resource | Resource |
| Iron Hill Settler | 4 Gold | Strong settler, +2 defense | Military |

**BUILDERS:**
| Name | Cost | Effect | Entry Type |
|------|------|--------|------------|
| Stone Mason | 2 Ore | Build structure | Resource |
| Master Architect | 4 Gold, 2 Ore | Build + upgrade existing | Production |
| Mine Engineer | 3 Ore | Build mine shaft | Mining |

**DEFENDERS:**
| Name | Cost | Effect | Entry Type |
|------|------|--------|------------|
| Militia | 1 Gold | Basic defender | Military |
| Shield Bearer | 2 Gold, 1 Ore | Strong defender | Military |
| Elite Guardian | 5 Gold | Powerful defender | Treasury |

**RESOURCE CARDS:**
| Name | Cost | Effect | Entry Type |
|------|------|--------|------------|
| Miner | 1 Gold | Generate 2 Ore/turn | Mining |
| Goldsmith | 2 Ore | Generate 3 Gold/turn | Treasury |
| Brewer | 1 Gold | Generate 2 Beer/turn | Supply |
| Farmer | 1 Gold | Generate 2 Food/turn | Supply |

**HEROES:**
| Name | Cost | Effect | Entry Type |
|------|------|--------|------------|
| Gimli | 6 Gold | Elite warrior, +3 to all adjacent defenders | Military |
| Thorin Oakenshield | 8 Gold | King, +2 to all entry points | Treasury |
| Gloin | 4 Gold | Smith, can upgrade defenders | Production |
| Balin | 3 Gold | Elder, heal adjacent cards | Any |

**MONSTERS (Enemy Cards):**
| Name | Threat | Effect | Entry Point |
|------|--------|--------|-------------|
| Goblin Scout | Low | Steal 1 resource | Any |
| Orc Raid | Medium | Attack defender, destroy weakest | Military |
| Warg Riders | Medium | Fast attack, bypass walls | Trade |
| Cave Troll | High | Heavy damage to defenses | Mining |
| Nazgul Sighting | High | Terror, weaken all defenders | All |
| Dragon Sighting | Epic | Major threat to Treasury | Treasury |
| The Balrog | BOSS | Endgame Day 300, all stats | All |

---

## 5. Resource Economy

Resources are generated by cards placed at entry points. Each turn, resources are collected based on deployed cards.

| Resource | Icon | Purpose | Generated By |
|----------|------|---------|-------------|
| **Gold** | 💰 | Currency, playing cards | Goldsmith, Trade |
| **Ore** | ⚒️ | Building, upgrades | Miner, Mine |
| **Beer** | 🍺 | Dwarf morale, hero recruitment | Brewer, Tavern |
| **Food** | 🍞 | Feed settlers, prevent starvation | Farmer, Marketplace |
| **Mithril** | 💎 | Rare, powerful upgrades | Deep mining |
| **Runes** | 🔮 | Magic, spells, special effects | Rune master |

### Resource Costs
Cards cost combinations of resources to play. Higher tier cards cost more.

---

## 6. Day/Night Cycle & Events

### Day Progression (Day 1-300)
- Each day has: **Dawn → Day → Dusk → Night** phases
- Resources collected each dawn
- Events trigger at dusk
- Night brings darker skies and increased danger

### Darkness Level (0.0 - 1.0)
- **Days 1-100:** 0.0-0.3 (Bright, safe)
- **Days 101-200:** 0.3-0.6 (Shadows grow)
- **Days 201-299:** 0.6-0.9 (Dark times)
- **Day 300:** 1.0 (The Balrog awakens - FINAL BATTLE)

### Random Events
Events occur at **specific entry points** based on darkness level.

**Common Events (Days 1-100):**
- Goblin raid (Mining)
- Merchant caravan (Trade)
- Festival (Supply)

**Uncommon Events (Days 101-200):**
- Orc ambush (Military)
- Warg attack (Trade routes)
- Dragon sighting (Treasury)

**Rare Events (Days 201-299):**
- Nazgul flyover (All entry points)
- Troll emergence (Mining)
- Shadow spreading (All)

**BOSS Event (Day 300):**
- **THE BALROG OF MORIA AWAKENS**
- All entry points under attack
- Must defeat to win the game

### Event Resolution
1. Event card appears at specific entry point
2. Player has option to defend or sacrifice entry point
3. If undefended, monsters deal damage
4. If defended, combat resolution occurs
5. Surviving cards remain, dead cards go to discard

---

## 7. Game Loop

### Turn Structure
1. **DAWN PHASE**: Collect resources from deployed cards
2. **DAY PHASE**: Play cards from hand to entry points
3. **DUSK PHASE**: Random event revealed at entry point
4. **NIGHT PHASE**: Combat resolution if event was attack
5. **END TURN**: Discard excess cards, advance day

### Playing Cards
1. Click a card in your hand (shows cost and entry type match)
2. Click an entry point on the world map
3. If resources sufficient and entry has space, card is placed
4. Card effect triggers immediately or passively each turn

### Winning & Losing
- **WIN:** Survive to Day 300 and defeat the Balrog
- **LOSE:** Kingdom destroyed (all entry points lost) or player reaches 0 HP

---

## 8. Entry Point Synergies

When cards match the entry point type, they receive bonuses:

| Entry Type | Matching Cards Get |
|------------|-------------------|
| Trade | +1 Gold per turn |
| Wealth | +2 Gold per turn |
| Resource | +1 Ore per turn |
| Supply | +1 Food per turn |
| Military | +1 Defense to all |
| Production | Can upgrade cards |
| Recruitment | Chance for bonus card |

### Building Upgrades
Cards at Production entry points can be upgraded:
- **Level 1:** Base card
- **Level 2:** +1 stat, costs 2 Ore
- **Level 3:** +2 stats, costs 4 Ore, 1 Mithril
- **Level 4:** +3 stats, costs 6 Ore, 2 Mithril

---

## 9. Architecture

```
src/
  main.rs              # Bevy app entry
  game/
    mod.rs
    state.rs           # GameState, DayCounter, Phase
    card.rs            # Card, CardType, CardEffect, Resources
    entry_point.rs     # EntryPoint, EntryType
    kingdom.rs         # Kingdom management, entry point states
    event.rs           # Random events, event deck
    combat.rs          # Combat resolution
    economy.rs         # Resource generation, costs
  data/
    mod.rs
    cards.rs           # CARD_REGISTRY — 50+ cards
    entries.rs         # ENTRY_POINT_REGISTRY — 8 entry points
    events.rs          # EVENT_REGISTRY — random events
  rendering/
    mod.rs
    world_map.rs       # World map UI, entry point rendering
    cards.rs           # Card UI
    ui.rs              # Top bar, resources, buttons
    effects.rs         # Day/night visuals, event overlays
```

---

## 10. Tech Stack

```toml
[dependencies]
bevy = "0.13"
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## 11. MVP Scope

**Phase 1 (Current):**
- 8 entry points on world map
- 30 starter cards (mix of all types)
- Basic resource economy
- Day/night cycle with events
- Card placement on entry points
- Simple combat/defense system

**Phase 2:**
- Card animations
- Sound effects
- Event deck with variety
- Upgrade system

**Phase 3:**
- Deck builder UI
- More cards
- Save/load

**Phase 4:**
- Full 300 cards
- Balrog boss fight
- Victory/defeat conditions
