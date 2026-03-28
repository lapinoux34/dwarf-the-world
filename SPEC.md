# Dwarf the World — Game Specification

## 1. Project Overview

**Type:** Deck-building survival game
**Engine:** Rust + Bevy (bevy_ecs, beev_ui)
**Theme:** Tolkien / Lord of the Rings dwarf fantasy
**Inspiration:** Don't Starve meets Hearthstone, set in Middle-earth
**Goal:** Survive Day 0–300 by managing a dwarven army across Tolkien-inspired locations as the shadow grows darker

The game evokes the atmosphere of Moria's darkness, Erebor's golden halls, and the escalating threat of Sauron's forces. Nights grow darker as Day 300 approaches — the Balrog stirs.

---

## 2. Visual Design

### Color Palette
- **Primary:** Gold (#D4AF37), Bronze (#CD7F32)
- **Backgrounds:** Deep brown (#2D1B0E), Cave darkness (#0D0A05), Charcoal (#1A1410)
- **Accent:** Torch orange (#FF6B00), Ember red (#8B0000)
- **UI Borders:** Mithril silver (#C0C0C0), Dwarven bronze (#B87333)

### Typography
- Card names: Bold fantasy serif
- Stats: Clean, readable

### Card Art Style
Gritty fantasy realism. AI generation prompts:
- Dwarves: "Lord of the Rings dwarf warrior detailed fantasy art", "Durin's Folk miner Moria illustration"
- Monsters: "Moria goblin dark fantasy illustration", "Tolkien troll stone illustration"

---

## 3. Core Data Structures

### Card
```
- id: u32
- name: String
- card_type: CardType (Dwarf, Monster, Event, Location, Resource)
- cost: u32
- attack: Option<u32>
- defense: Option<u32>
- effect: Option<CardEffect>
- art_prompt: String  # AI art generation prompt
- faction: Option<DwarfFaction>
```

### CardType
```
Dwarf, Monster, Event, Location, Resource
```

### DwarfFaction
```
Erebor,        # Thorin Company, treasure-focused
Moria,         # Durin's Folk, mining, darker
Dale,          # Allies, traders
IronHills      # Warriors, defense
```

### BoardLocation
```
- id: u32
- name: String
- description: String
- max_cards: u32
- location_type: LocationZone
```

### LocationZone
```
Erebor,        # The Lonely Mountain - treasure room
Moria,         # Mines of Moria - cavern zones
Dale,          # Dale Marketplace - trade zone
HelmsDeep,     # Helm's Deep - defensive fortress
Mirkwood       # Mirkwood Forest - dangerous wild
```

### GameState
```
- day: u32          # 0–300
- phase: Phase      # Draw, Play, Combat, EndTurn
- deck: Deck
- hand: Vec<Card>
- discard: Vec<Card>
- board: HashMap<BoardLocation, Vec<Card>>
- resources: Resources
- darkness_level: f32  # 0.0 (dawn) -> 1.0 (Balrog awakens)
```

### Phase
```
Draw, Play, Combat, EndTurn
```

### Resources
```
- gold: u32
- mithril: u32
- provisions: u32   # Food/ale for dwarves
- runestones: u32
```

---

## 4. Board Locations

| ID | Name | Description | Max Cards | Theme |
|----|------|-------------|-----------|-------|
| 1 | The Lonely Mountain (Erebor) | The great dwarf kingdom. Treasure vaults overflow with gold. | 5 | Gold/treasure |
| 2 | Mines of Moria | Dark caverns beneath the mountains. Rich in mithril, rich in danger. | 5 | Cave darkness |
| 3 | Dale Marketplace | Where dwarves and men trade goods and ale. | 4 | Trade/warmth |
| 4 | Helm's Deep | The fortress of Rohan — allies defend together. | 4 | Defense/fortress |
| 5 | Mirkwood Forest | Twisted trees and spider-infested shadows. | 5 | Dark wilderness |

---

## 5. Starter Cards (20)

### Dwarves — Erebor (Thorin Company)
| ID | Name | Class | Cost | ATK | DEF | Faction |
|----|------|-------|------|-----|-----|---------|
| 1 | Thorin Oakenshield | Leader | 5 | 6 | 5 | Erebor |
| 2 | Dwalin | Warrior | 4 | 5 | 4 | Erebor |
| 3 | Balin | Elder | 3 | 3 | 4 | Erebor |
| 4 | Bifur | Miner | 2 | 2 | 3 | Erebor |
| 5 | Bombur | Defender | 3 | 2 | 5 | Erebor |

### Dwarves — Moria (Durin's Folk)
| ID | Name | Class | Cost | ATK | DEF | Faction |
|----|------|-------|------|-----|-----|---------|
| 6 | Gimli | Elite Warrior | 5 | 7 | 5 | Moria |
| 7 | Gloin | Smith | 3 | 3 | 4 | Moria |
| 8 | Oin | Rune Master | 3 | 2 | 4 | Moria |
| 9 | Nori | Shadow Walker | 3 | 4 | 2 | Moria |
| 10 | Dori | Strong Arm | 2 | 3 | 3 | Moria |

### Monsters — Moria Depths
| ID | Name | Cost | ATK | DEF |
|----|------|------|-----|-----|
| 11 | Moria Goblin | 1 | 2 | 1 |
| 12 | Cave Troll | 4 | 6 | 5 |
| 13 | Dol Guldur Orc | 2 | 3 | 2 |
| 14 | Mirkwood Spider | 3 | 4 | 3 |
| 15 | Warg Rider | 3 | 5 | 2 |
| 16 | Orc Archer | 2 | 3 | 1 |
| 17 | Cave Bat Swarm | 1 | 2 | 1 |
| 18 | Moria Goblin Shaman | 3 | 2 | 3 |
| 19 | Troll | 5 | 7 | 6 |
| 20 | The Balrog of Moria | 10 | 12 | 10 |

---

## 6. Day/Night Cycle & Darkness

- **Days 1-100:** Bright, golden tones. Normal gameplay.
- **Days 101-200:** Shadows lengthen. Darkness level 0.3-0.6. More monsters spawn.
- **Days 201-299:** Dark. Darkness level 0.7-0.9. Balrog stirs.
- **Day 300:** The Balrog awakens. Final battle. Game climax.

The `darkness_level` affects:
- Board background color (darker = more darkness)
- Monster spawn rates
- Available card effects

---

## 7. Game Loop

1. **Draw Phase**: Draw 1 card from deck
2. **Play Phase**: Click a card in hand, then click a board location to place it
3. **Combat Phase**: All dwarves vs. all monsters on each location resolve combat
4. **End Turn**: Discard excess cards, increment day, darkness grows, spawn monster event

### Combat Resolution
- Dwarves attack monsters on the same location
- If no monsters, dwarf attacks the "base" (player takes damage = remaining ATK)
- Monsters attack dwarves on the same location
- If no dwarves, monsters attack the base
- Defense reduces incoming damage (ATK - DEF, minimum 0)
- Cards at 0 HP are destroyed → discard pile

---

## 8. Rendering

### Card Visual (Hearthstone-inspired but darker)
- Size: 120×170 px
- Frame: Dark bronze with gold filigree border
- Top: Card name + cost gem (red for monsters, blue for dwarves)
- Middle: Art area (colored gradient placeholder by faction/type)
- Bottom: ATK (sword icon) / DEF (shield icon)
- Faction badge in corner

### Board
- Dark stone/mountain background with torch-lit edges
- 5 clickable zone rectangles with Tolkien-themed borders
- Zone label + card count
- Cards fan out in zone

### UI Overlay
- Top bar: Day counter, Darkness indicator, Resource display
- Bottom: Hand area (fanned cards)
- Buttons: End Turn, View Deck, View Discard

---

## 9. Architecture

```
src/
  main.rs           # Bevy app entry
  game/
    mod.rs
    state.rs        # GameState, Phase, Resources
    deck.rs         # Deck/Hand/Discard pile management
    card.rs         # Card, CardType, CardEffect, DwarfFaction
    board.rs        # BoardLocation, placement
    combat.rs       # Combat resolution
    economy.rs      # Resource management
  data/
    mod.rs
    cards.rs        # CARD_REGISTRY — all starter cards
    locations.rs    # LOCATION_REGISTRY — 5 Tolkien zones
  rendering/
    mod.rs
    cards.rs        # Card UI bundle, CardPlugin
    board.rs        # Board UI bundle, BoardPlugin
    ui.rs           # Top bar, buttons, overlays, darkness effect
```

---

## 10. MVP Dependencies

```toml
[dependencies]
bevy = "0.13"
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## 11. Future Phases

- **Phase 2**: Card animations, day/night transitions, sound effects
- **Phase 3**: Deck builder UI, card acquisition through gameplay
- **Phase 4**: Save/load, campaign progression
- **Phase 5**: 300+ cards, events, complex effects, Balrog boss mechanics
