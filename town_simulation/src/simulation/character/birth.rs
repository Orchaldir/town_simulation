use crate::generation::number::RandomNumberGenerator;
use crate::model::character::CharacterId;
use crate::usecase::character::birth::birth;
use crate::usecase::character::marriage::get_married_couples;
use crate::SimulationData;

const BIRTH: u32 = 3;

pub fn simulate_birth(data: &mut SimulationData, rng: &RandomNumberGenerator) {
    for (id0, id1) in calculate_expecting(&data, rng, 45, 10) {
        birth(data, id0, id1);
    }
}

fn calculate_expecting(
    data: &&mut SimulationData,
    rng: &RandomNumberGenerator,
    max_age: u32,
    chance_of_birth: u32,
) -> Vec<(CharacterId, CharacterId)> {
    let mut expecting = Vec::new();

    for (id0, id1) in get_married_couples(&data.character_manager) {
        let character0 = data.character_manager.get(id0).unwrap();

        if character0.is_dead() {
            continue;
        }

        let character1 = data.character_manager.get(id1).unwrap();

        if character1.is_dead() {
            continue;
        }

        let age0 = character0.get_age(data.date);
        let age1 = character1.get_age(data.date);
        let age = age0.max(age1);

        if age > max_age {
            continue;
        }

        let roll = rng.roll_d100(data.date.get_year(), id0.id(), BIRTH);

        if roll < chance_of_birth {
            expecting.push((id0, id1));
        }
    }

    expecting
}
