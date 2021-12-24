use crate::generation::number::RandomNumberGenerator;
use crate::usecase::character::birth::{birth, set_birth_date};
use crate::usecase::character::marriage::get_married_couples;
use crate::usecase::character::{set_gender_based_on_id, set_generated_name};
use crate::SimulationData;

const BIRTH: u32 = 3;

pub fn simulate_birth(data: &mut SimulationData, rng: &RandomNumberGenerator) {
    let max_age = 45;
    let chance_of_birth = 20;
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

    for (id0, id1) in expecting {
        let child_id = birth(&mut data.character_manager, id0, id1);

        println!(
            "Characters {} & {} get child {}",
            id0.id(),
            id1.id(),
            child_id.id()
        );

        set_birth_date(&mut data.character_manager, child_id, data.date);
        set_gender_based_on_id(&mut data.character_manager, child_id);
        set_generated_name(
            &mut data.character_manager,
            &data.character_name_generator,
            child_id,
        );
    }
}
