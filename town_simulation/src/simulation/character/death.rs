use crate::generation::number::RandomNumberGenerator;
use crate::usecase::character::death::death;
use crate::SimulationData;

const DEATH: u32 = 1;

pub fn simulate_death(data: &mut SimulationData, rng: &RandomNumberGenerator) {
    let min_age_for_death = 60;
    let mut dying = Vec::new();

    for character in data.character_manager.get_all() {
        if character.is_dead() {
            continue;
        }

        let age = character.get_age(data.date);

        if age < min_age_for_death {
            continue;
        }

        let risk_of_death = age - min_age_for_death;
        let roll = rng.roll_d100(data.date.get_year(), character.id().id(), DEATH);

        if roll < risk_of_death {
            println!(
                "Character {:?} (age {}) dies with {}",
                character.id(),
                age,
                roll
            );
            dying.push(*character.id());
        }
    }

    for id in dying {
        death(&mut data.character_manager, id, data.date);
    }
}
