use crate::generation::number::RandomNumberGenerator;
use crate::model::character::CharacterId;
use crate::usecase::character::death::death;
use crate::SimulationData;

const DEATH: u32 = 1;

pub fn simulate_death(data: &mut SimulationData, rng: &RandomNumberGenerator) {
    for id in calculate_dying(data, rng, 60) {
        death(&mut data.character_manager, id, data.date);
    }
}

fn calculate_dying(
    data: &mut SimulationData,
    rng: &RandomNumberGenerator,
    min_age: u32,
) -> Vec<CharacterId> {
    let mut dying = Vec::new();

    for character in data.character_manager.get_all() {
        if character.is_dead() {
            continue;
        }

        let age = character.get_age(data.date);

        if age < min_age {
            continue;
        }

        let risk_of_death = (age - min_age) / 2;
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

    dying
}
