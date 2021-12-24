use crate::generation::number::RandomNumberGenerator;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::usecase::character::marriage::{get_unmarried, marry};
use crate::SimulationData;
use std::collections::HashSet;

const MARRIAGE: u32 = 2;

pub fn simulate_marriage(data: &mut SimulationData, rng: &RandomNumberGenerator) {
    let min_age = 18;
    let max_age = 60;
    let base_chance = 20;
    let marriageable = get_marriageable(data, min_age, max_age);
    let selected_characters =
        select_characters_to_marry(data, rng, &marriageable, min_age, base_chance);
    let mut remaining: HashSet<CharacterId> = marriageable
        .difference(&selected_characters)
        .copied()
        .collect();

    for id in selected_characters {
        if let Some(spouse) = select_spouse(&data.character_manager, id, &remaining) {
            marry(&mut data.character_manager, id, spouse);
            remaining.remove(&spouse);
        }
    }
}

fn get_marriageable(data: &SimulationData, min_age: u32, max_age: u32) -> HashSet<CharacterId> {
    let mut marriageable = HashSet::new();

    for id in get_unmarried(&data.character_manager) {
        let character = data.character_manager.get(id).unwrap();

        if character.is_dead() {
            continue;
        }

        let age = character.get_age(data.date);

        if age < min_age || age > max_age {
            continue;
        }

        marriageable.insert(id);
    }
    marriageable
}

fn select_characters_to_marry(
    data: &SimulationData,
    rng: &RandomNumberGenerator,
    marriageable: &HashSet<CharacterId>,
    min_age: u32,
    base_chance: u32,
) -> HashSet<CharacterId> {
    let mut selected = HashSet::new();

    for id in marriageable {
        let character = data.character_manager.get(*id).unwrap();
        let age = character.get_age(data.date);
        let chance_of_marriage = base_chance - (age - min_age);
        let roll = rng.roll_d100(data.date.get_year(), id.id(), MARRIAGE);

        if roll < chance_of_marriage {
            println!(
                "Character {:?} (age {}) marries with {}",
                id.id(),
                age,
                roll
            );
            selected.insert(*id);
        } else {
            println!(
                "Character {:?} (age {}) doesn't marry with {}",
                id.id(),
                age,
                roll
            );
        }
    }

    selected
}

fn select_spouse(
    manager: &CharacterMgr,
    id: CharacterId,
    candidates: &HashSet<CharacterId>,
) -> Option<CharacterId> {
    let character = manager.get(id).unwrap();

    candidates
        .iter()
        .map(|&candidate_id| manager.get(candidate_id).unwrap())
        .filter(|&candidate| character.gender().is_reverse(*candidate.gender()))
        .map(|candidate| *candidate.id())
        .next()
}
