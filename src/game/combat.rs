use super::card::Card;

#[derive(Debug, Clone)]
pub struct CombatResult {
    pub card_id: u32,
    pub damage_dealt: u32,
    pub damage_taken: u32,
    pub destroyed: bool,
}

pub fn resolve_combat(
    attackers: &[Card],
    defenders: &mut [Card],
) -> (Vec<CombatResult>, Vec<CombatResult>) {
    let mut attacker_results = Vec::new();
    let mut defender_results = Vec::new();

    let mut defender_idx = 0;

    for attacker in attackers {
        if defender_idx >= defenders.len() {
            break;
        }

        let defender = &mut defenders[defender_idx];

        let attack_power = attacker.attack.unwrap_or(0);
        let defense_power = defender.defense.unwrap_or(0);

        let damage_to_defender = attack_power.saturating_sub(defense_power);
        let damage_to_attacker = defense_power.saturating_sub(attack_power);

        defender.defense = Some(defense_power.saturating_sub(damage_to_defender));

        defender_results.push(CombatResult {
            card_id: defender.id,
            damage_dealt: attack_power,
            damage_taken: damage_to_attacker,
            destroyed: defender.defense.unwrap_or(0) == 0,
        });

        attacker_results.push(CombatResult {
            card_id: attacker.id,
            damage_dealt: 0,
            damage_taken: damage_to_attacker,
            destroyed: false,
        });

        defender_idx += 1;
    }

    (attacker_results, defender_results)
}

pub fn apply_combat_results(
    cards: &mut Vec<Card>,
    results: &[CombatResult],
) -> Vec<Card> {
    let destroyed_ids: Vec<u32> = results
        .iter()
        .filter(|r| r.destroyed)
        .map(|r| r.card_id)
        .collect();

    cards.retain(|c| !destroyed_ids.contains(&c.id));
    cards.into_iter().filter(|c| !destroyed_ids.contains(&c.id)).map(|c| c.clone()).collect()
}
