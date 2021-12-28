use crate::model::character::CharacterId;
use crate::usecase::building::relocate::join_parents_home;
use crate::usecase::character::birth::{birth, set_birth_date};
use crate::usecase::character::{set_gender_based_on_id, set_generated_name};
use crate::SimulationData;

pub fn generate_child(
    data: &mut SimulationData,
    id0: CharacterId,
    id1: CharacterId,
) -> CharacterId {
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

    join_parents_home(data, vec![child_id], id0);

    child_id
}
