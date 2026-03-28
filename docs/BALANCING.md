# Dwarf the World — Game Balance Philosophy

**Version:** 1.0  
**Date:** 2026-03-28  
**Principe:** "Dur mais pas impossible" — Fun à mourir, satisfaisant à gagner

---

## 1. Principe Fondamental

> Chaque run doit se terminer par "**J'aurais pu jouer mieux**" — jamais "**C'était n'importe quoi**"

- La mort est normale, elle fait partie du roguelike
- Mais chaque défaite doit être une leçon, pas de la frustration
- La victoire à J300 doit être **méritée et héroïque**

---

## 2. Courbe de Difficulté

### Phase Débutante (J1-J30) — Apprentissage
- Monsters: ATK 1-3, DEF 1-3
- Events: mineurs, facilement évitables
- Ressources: généreuses, le joueur experiment les mécaniques
-Conseil: fais des erreurs, c'est gratuit

### Phase Intermédiaire (J31-J100) — Montée
- Monsters: ATK 4-6, DEF 3-5
- Events: nontrivial, demande des décisions
- Ressources: tendues mais gérables
- Conseil: commence à mourir

### Phase Avancée (J101-J250) — Urgence
- Monsters: ATK 7-10, DEF 6-9
- Events: dangereux, conséquences réelles
- Ressources: précieuses, choix critiques
- Conseil: chaque carte compte

### Phase Finale (J251-J300) — Désespoir
- Monsters: ATK 10-15, DEF 8-12
- Events: catastrophes, apocalypse naine
- J300: **BALROG** (le vrai test, extremely hard mais possible)

---

## 3. Équilibre des Ressources

### Gold
| Phase | Revenu/turn | Usage principal |
|-------|-------------|-----------------|
| J1-J30 | 5-8 | Recruter des nains |
| J31-J100 | 8-12 | Montée en puissance |
| J101-J250 | 10-15 | Combat et survie |
| J251-J300 | 12-20 | Armageddon |

### Ore
- Nécessaire pour les bâtiments et armes
- Ne pas en manque CRITICAL pendant les 30 premiers jours
- Après J100: devient précieux

### Beer
- Moral des nains
- Si beer = 0 pendant 3 tours consécutifs: désertion
- Mais pas de panique avant J50

### Runes
- Magie = late game
- J1-J50: quasi inutile
- J51+: de plus en plus critique

---

## 4. Équilibre des Cartes

### Règle n°1: Coût = Puissance / 2 + 3

```
Coût idéal d'une carte = (ATK + DEF) / 2 + 3
```

| Stats totaux | Coût minimum | Coût maximum |
|--------------|--------------|--------------|
| 2-4 | 2 | 3 |
| 5-7 | 3 | 4 |
| 8-10 | 4 | 6 |
| 11-14 | 6 | 8 |
| 15+ | 8 | 10+ |

### Règle n°2: Pas de carte useless

Chaque carte doit avoir un **cas d'usage**:

| Type | Cas d'usage |
|------|-------------|
| Dwarf Warrior | Combat direct |
| Dwarf Miner | Génération ore |
| Dwarf Brewer | Génération beer, moral |
| Dwarf Smith | Buff ATK |
| Dwarf Ranger | Utility, defense |
| Dwarf Mage | Damage dealer |
| Monster | Threat, loot |
| Building | Permanent buff |
| Event | Tweak la partie |

### Règle n°3: Pas de carte OP

Si une carte fait tout: buff + heal + damage + draw → **TROP PUISSANTE**

Vérification:
- Une carte qui résout tous les problèmes = OP
- Une carte qui demande du setup mais paye enormément = **BON DESIGN**

---

## 5. Équilibre des Zones

### Bonus de zone (ce qui existe):
- Dale City Gates: +1 gold/carte
- Erebor Treasury: +1 gold/nain
- Moria Mines: +1 ore/carte
- Dale Marketplace: +1 beer (3+ cartes)
- Mountain Pass: +2 DEF à tous les nains
- River Dock: +1 carte au play
- Dwarven Forge: +1 ATK aux nains
- Tavern Gate: chance héros gratuit
- Mirkwood Border: danger + loot
- Dale Farmlands: prevents beer shortage

### Équilibre recommandé:
- **Mountain Pass** = meilleure zone défensive (valable)
- **Mirkwood** = high risk/high reward (valable)
- **Forge** = late-game powerhouse (valable)
- **Treasury** = early game safe (un peu forte, nerf si problèmes)

---

## 6. Système de Combat

### Règles de combat:

1. **ATK - DEF = damage dealt** (minimum 0)
2. Pas de one-shot (DEF minimum 1 par défaut, même les weak cards)
3. Les nains meurent defending, pas tous d'un coup
4. Si zone menacée et non défendue: conséquences progressives

### Threat Level (niveau de menace):
| Level | Description |
|-------|-------------|
| 0 | Pas de menace |
| 1-3 | Monsters mineurs, facile |
| 4-6 | Challenge modéré |
| 7-9 | Dangerous, preparation nécessaire |
| 10+ | Boss de zone |

### Balrog (J300):
- ATK: 30
- DEF: 25
- HP: 100
- Phase 1: 3 tours pour setup
- Phase 2: Enrage si pas tué en 5 tours
- Reward: Victoire + "Balrog Slayer" badge

---

## 7. Test d'Équilibre

### Playtest automatique (checklist):

- [ ] J1-J10: peux-tu survivre sans stress?
- [ ] J30: as-tu assez de resources pour continuer?
- [ ] J50: la difficulté a-t-elle augmenté?
- [ ] J100: as-tu encore du fun?
- [ ] J200: chaque décision est-elle importante?
- [ ] J300: le Balrog est-il hard mais pas impossible?

### Feedback markers:
| Symptôme | Problème | Solution |
|----------|----------|----------|
| Mort en J10-J20 | Early game trop dur | Buff resources early, nerf monsters early |
| Facile jusqu'à J100 | Difficulté monte trop lentement | +2 ATK aux monsters à J50, J75 |
| "Unfair death" | Un mechanic est cassé | Revoir les events qui one-shot |
| Toujours la même strat | Méta solved | Nerf les cartes trop dominantes, buff les alternatives |
| Victoire impossible à J300 | Balrog trop dur | Ajuster HP/ATK, ajuster la courbe |

---

## 8. Fun Hard Design

### Ce qui rend un roguelike fun-hard:

1. **Prises de décision interessantes** — pas "choisis A ou B, les deux marchent"
2. **Consequences réelles** — mes choix ont du poids
3. **Ressources précieuses** — je dois choisir, pas tout faire
4. **Progression visible** — mes nains deviennent plus forts
5. **Variance controllée** — le random est là mais pas injuste

### Ce qui rend un roguelike pas fun:

1. **RNG decides** — pas le skill
2. **One-shot deaths** — pas de counterplay
3. **No choice** — la seule option est évidente
4. **Stomp or get stomped** — toujours easy ou toujours lose
5. **Grind** — farm pour progresser

---

## 9. Tuning Numbers (suggested)

```
STARTING_RESOURCES:
  gold: 20
  ore: 10
  beer: 10
  runes: 0
  food: 15

CARDS_PER_DRAW: 1
MAX_HAND_SIZE: 8
ZONE_DEFENSE_BASE: 0

DAILY_EVENT_CHANCE: 30%  (every day, 30% chance an event happens)
DAILY_THREAT_CHANCE: 20% (every day, 20% chance a zone gets threatened)

BALROG_STATS:
  attack: 30
  defense: 25
  hp: 100
  enrage_turns: 5
```

---

## 10. Checklist Pré-release

- [ ] 50 cards — each tested, none OP
- [ ] All 10 zones — each useful
- [ ] Day 1-30 — learnable
- [ ] Day 31-100 — engaging
- [ ] Day 101-250 — tense
- [ ] Day 251-299 — desperate
- [ ] Day 300 Balrog — earned victory
- [ ] No one-shot deaths
- [ ] No infinite combo exploits
- [ ] Resources always meaningful
- [ ] Events always manageable
- [ ] Variance within fun range

---

_End of Balance Philosophy — Dwarf the World_
