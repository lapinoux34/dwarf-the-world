use super::card::Card;

#[derive(Debug, Clone)]
pub struct CombatResult {
    pub card: Card,
    pub damage_taken: u32,
    pub destroyed: bool,
}

pub fn resolve_combat_on_location(
    dwarves: &mut Vec<Card>,
    monsters: &mut Vec<Card>,
) -> (Vec<CombatResult>, Vec<CombatResult>) {
    let mut dwarf_results = Vec::new();
    let mut monster_results = Vec::new();

    // Pair dwarves with monsters (simple: first dwarf vs first monster, etc.)
    let mut dwarf_idx = 0;
    let mut monster_idx = 0;

    while dwarf_idx < dwarves.len() && monster_idx < monsters.len() {
        let dwarf = &dwarves[dwarf_idx];
        let monster = &mut monsters[monster_idx];

        let dwarf_attack = dwarf.get_attack();
        let monster_attack = monster.get_attack();

        let monster_defense = monster.get_defense();
        let dwarf_defense = dwarf.get_defense();

        // Dwarf attacks monster
        let dmg_to_monster = dwarf_attack.saturating_sub(monster_defense);
        let current_hp = monster.get_defense();
        let new_hp = current_hp.saturating_sub(dmg_to_monster);
        monster.defense = Some(new_hp);

        // Monster attacks dwarf
        let dmg_to_dwarf = monster_attack.saturating_sub(dwarf_defense);
        let dwarf_hp = dwarf.get_defense();
        let new_dwarf_hp = dwarf_hp.saturating_sub(dmg_to_dwarf);

        // Update dwarf defense (as HP)
        let mut updated_dwarf = dwarves[dwarf_idx].clone();
        updated_dwarf.defense = Some(new_dwarf_hp);

        monster_results.push(CombatResult {
            card: monsters[monster_idx].clone(),
            damage_taken: dmg_to_monster,
            destroyed: new_hp == 0,
        });

        dwarf_results.push(CombatResult {
            card: updated_dwarf.clone(),
            damage_taken: dmg_to_dwarf,
            destroyed: new_dwarf_hp == 0,
        });

        dwarves[dwarf_idx] = updated_dwarf;

        dwarf_idx += 1;
        monster_idx += 1;
    }

    // Leftover dwarves attack base (nothing in MVP)
    // Leftover monsters attack base (nothing in MVP)

    (dwarf_results, monster_results)
}

pub fn apply_combat_results(
    dwarves: &mut Vec<Card>,
    monsters: &mut Vec<Card>,
    dead_dwarves: &[CombatResult],
    dead_monsters: &[CombatResult],
) {
    // Remove destroyed cards
    let dead_dwarf_ids: Vec<u32> = dead_dwarves.iter().filter(|r| r.destroyed).map(|r| r.card.id).collect();
    let dead_monster_ids: Vec<u32> = dead_monsters.iter().filter(|r| r.destroyed).map(|r| r.card.id).collect();

    dwarves.retain(|d| !dead_dwarf_ids.contains(&d.id));
    monsters.retain(|m| !dead_monster_ids.contains(&m.id));
}
