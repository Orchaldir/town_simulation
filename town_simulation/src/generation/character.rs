use crate::generation::name::character::CharacterNameGenerator;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::model::time::Date;
use crate::usecase::character::birth::{birth, set_birth_date};
use crate::usecase::character::{set_gender_based_on_id, set_generated_name};

pub fn generate_child(
    manager: &mut CharacterMgr,
    name_generator: &CharacterNameGenerator,
    id0: CharacterId,
    id1: CharacterId,
    date: Date,
) -> CharacterId {
    let child_id = birth(manager, id0, id1);

    println!(
        "Characters {} & {} get child {}",
        id0.id(),
        id1.id(),
        child_id.id()
    );

    set_birth_date(manager, child_id, date);
    set_gender_based_on_id(manager, child_id);
    set_generated_name(manager, name_generator, child_id);

    child_id
}
