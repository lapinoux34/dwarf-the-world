# Dwarf the World — Game Specification

## 1. Project Overview

**Type:** Deck-building survival game
**Engine:** Rust + Bevy (bevy_ecs, bevy_ui)
**Inspiration:** Don't Starve meets Hearthstone
**Goal:** Survive Day 0–300 by managing a dwarven army across multiple board locations

Players draw cards, place dwarves and monsters on board zones, resolve combat, and manage resources (gold, ore, beer, runes) to survive escalating daily challenges.

---

## 2. Core Data Structures

### Card
```
- id: u32
- name: String
- card_type: CardType (Dwarf, Monster, Event, Location, Resource)
- cost: u32
- attack: Option<u32>
- defense: Option<u32>
- effect: Option<CardEffect>
- art_path: String
```

### CardType
```
Dwarf, Monster, Event, Location, Resource
```

### DwarfClass
```
Warrior, Miner, Brewer, Smith, Ranger, Mage
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
MineEntrance, Forge, Tavern, MountainPeak, UndergroundCavern
```

### GameState
```
- day: u32          // 0–300
- phase: Phase      // Draw, Play, Combat, EndTurn
- deck: Deck
- hand: Hand
- discard: Vec<Card>
- board: HashMap<BoardLocation, Vec<Card>>
- resources: Resources
```

### Phase
```
Draw, Play, Combat, EndTurn
```

### Resources
```
- gold: u32
- ore: u32
- beer: u32
- runes: u32
```

---

## 3. Board Locations (MVP — 5 zones)

| ID | Name | Description | Max Cards |
|----|------|-------------|-----------|
| 1 | Mine Entrance | Where dwarves first emerge | 5 |
| 2 | The Forge | Weapons and armor crafting | 4 |
| 3 | The Tavern | Rest, beer, and merriment | 4 |
| 4 | Mountain Peak | High ground, ranger territory | 3 |
| 5 | Underground Cavern | Dark depths, dangerous creatures | 5 |

---

## 4. Starter Cards (MVP — 20 cards)

### Dwarves (10)
| ID | Name | Class | Cost | ATK | DEF |
|----|------|-------|------|-----|-----|
| 1 | Thorin Ironbeard | Warrior | 3 | 4 | 3 |
| 2 | Dwalin the Bold | Warrior | 4 | 5 | 4 |
| 3 | Balin Goldminer | Miner | 2 | 2 | 2 |
| 4 | Bifur Bronzehaft | Miner | 3 | 3 | 4 |
| 5 | Dori Silvermug | Brewer | 2 | 1 | 3 |
| 6 | Nori Shadowstep | Brewer | 3 | 3 | 2 |
| 7 | Gloin Fireforge | Smith | 4 | 4 | 5 |
| 8 | Oin RuneReader | Smith | 3 | 2 | 4 |
| 9 | Bombur Ironshield | Ranger | 3 | 3 | 3 |
| 10 | Dain Stonefoot | Mage | 5 | 6 | 4 |

### Monsters (10)
| ID | Name | Cost | ATK | DEF |
|----|------|------|-----|-----|
| 11 | Goblin Scout | 1 | 2 | 1 |
| 12 | Goblin Raider | 2 | 3 | 2 |
| 13 | Cave Troll | 4 | 6 | 5 |
| 14 | Mountain Troll | 5 | 7 | 6 |
| 15 | Cave Bat | 1 | 2 | 1 |
| 16 | Dire Wolf | 3 | 4 | 3 |
| 17 | Stone Golem | 4 | 3 | 7 |
| 18 | Goblin Shaman | 3 | 2 | 3 |
| 19 | Mountain Ogre | 5 | 6 | 5 |
| 20 | Young Dragon | 7 | 8 | 7 |

---

## 5. Game Loop (MVP)

1. **Draw Phase**: Draw 1 card from deck
2. **Play Phase**: Click a card in hand, then click a board location to place it
3. **Combat Phase**: All dwarves vs. all monsters on each location resolve combat
4. **End Turn**: Discard excess cards, increment day, spawn monster event

### Combat Resolution
- Dwarves attack monsters on the same location
- If no monsters, dwarf attacks the "base" (player takes damage = remaining ATK)
- Monsters attack dwarves on the same location
- If no dwarves, monsters attack the base
- Defense reduces incoming damage (ATK - DEF, minimum 0)
- Cards at 0 HP are destroyed → discard pile

### Turn Flow
```
StartTurn → DrawPhase → PlayPhase → CombatPhase → EndTurn → (next day)
```

---

## 6. Rendering (MVP)

### Card Visual
- Size: 120×170 px
- Frame: dark border with gold accent
- Top: Card name + cost gem
- Middle: Art placeholder (colored rect by type)
- Bottom: ATK (sword icon) / DEF (shield icon)
- Type badge in corner

### Board
- Dark stone/mountain background
- 5 clickable zone rectangles
- Zone label + card count
- Cards fan out in zone

### UI Overlay
- Top bar: Day counter, Resource display
- Bottom: Hand area (fanned cards)
- Buttons: End Turn, View Deck, View Discard

---

## 7. Architecture

```
src/
  main.rs           # Bevy app entry
  game/
    mod.rs
    state.rs        # GameState, Phase, Resources
    deck.rs         # Deck/Hand/Discard pile management
    card.rs         # Card, CardType, CardEffect
    board.rs        # BoardLocation, placement
    combat.rs       # Combat resolution
    economy.rs      # Resource management
  data/
    mod.rs
    cards.rs        # CARD_REGISTRY — all 20 starter cards
    locations.rs    # LOCATION_REGISTRY — 5 zones
  rendering/
    mod.rs
    cards.rs        # Card UI bundle, CardPlugin
    board.rs        # Board UI bundle, BoardPlugin
    ui.rs           # Top bar, buttons, TurnIndicator
```

---

## 8. MVP Dependencies

```toml
[dependencies]
bevy = "0.16"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rand = "0.8"
```

---

## 9. Future Phases (Out of Scope for MVP)

- **Phase 2**: Card animations, day/night visuals, sound
- **Phase 3**: Deck builder UI, card acquisition
- **Phase 4**: Save/load, campaign progression
- **Phase 5**: 300+ cards, events, complex effects
