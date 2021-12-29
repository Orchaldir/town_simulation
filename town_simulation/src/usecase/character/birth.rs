use crate::model::character::relation::character::family::RelativeType;
use crate::model::character::relation::character::family::RelativeType::*;
use crate::model::character::relation::character::CharacterRelationType::*;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::model::time::Date;
use crate::usecase::building::relocate::join_parents_home;
use crate::usecase::character::relation::get::*;
use crate::usecase::character::{
    add_relation, add_relations, set_gender_based_on_id, set_generated_name,
};
use crate::SimulationData;
use std::collections::HashSet;

pub fn birth(data: &mut SimulationData, id0: CharacterId, id1: CharacterId) -> CharacterId {
    let child_id = birth_with_relations(&mut data.character_manager, id0, id1);

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

pub fn birth_with_relations(
    manager: &mut CharacterMgr,
    father: CharacterId,
    mother: CharacterId,
) -> CharacterId {
    let parents = [father, mother].into();
    let child = manager.create();
    let siblings = get_shared_children(manager, father, mother);
    let grandparents = combine(&parents, |id| get_parents(manager, id));
    let piblings = combine(&parents, |id| get_siblings(manager, id));
    let cousins = combine(&piblings, |id| get_children(manager, id));

    add_relations(manager, child, &grandparents, Relative(GrandChild));
    add_relations(manager, child, &cousins, Relative(Cousin));
    add_relations(manager, child, &piblings, Relative(Nibling));
    add_relations(manager, child, &siblings, Relative(Sibling));
    add_relations(manager, child, &parents, Relative(Child));

    let mut relatives = HashSet::new();
    relatives.extend(parents.clone());
    relatives.extend(siblings.clone());
    relatives.extend(grandparents.clone());
    relatives.extend(piblings.clone());
    relatives.extend(cousins.clone());

    add_in_laws(manager, child, &grandparents, &relatives, GrandChild);
    add_in_laws(manager, child, &cousins, &relatives, Cousin);
    add_in_laws(manager, child, &piblings, &relatives, Nibling);
    add_in_laws(manager, child, &siblings, &relatives, Sibling);
    add_in_laws(manager, child, &parents, &relatives, Child);

    child
}

pub fn set_birth_date(manager: &mut CharacterMgr, id: CharacterId, date: Date) {
    manager.get_mut(id).unwrap().set_birth_date(date)
}

pub fn get_birth_date(manager: &CharacterMgr, id: CharacterId) -> &Date {
    manager.get(id).unwrap().birth_date()
}

fn add_in_laws(
    manager: &mut CharacterMgr,
    character: CharacterId,
    targets: &HashSet<CharacterId>,
    relatives: &HashSet<CharacterId>,
    relative_type: RelativeType,
) {
    let relation_type = InLaw(relative_type);

    for target in targets {
        for spouse in get_spouses(manager, *target) {
            if !relatives.contains(&spouse) {
                add_relation(manager, character, spouse, relation_type);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::building::usage::BuildingUsage;
    use crate::usecase::building::build::build;
    use crate::usecase::building::occupancy::{get_building_occupied_by, get_occupants};
    use crate::usecase::building::relocate::relocate_to_house;
    use crate::usecase::character::marriage::marry;
    use crate::util::assert::assert;

    #[test]
    fn set_birth_date_of_character() {
        let mut manager = CharacterMgr::default();
        let id = manager.create();
        let date = Date::new(42);

        set_birth_date(&mut manager, id, date);

        assert_eq!(get_birth_date(&manager, id), &date)
    }

    #[test]
    fn child_live_in_their_parents_home() {
        let mut data = SimulationData::default();

        let parent_id0 = data.character_manager.create();
        let parent_id1 = data.character_manager.create();

        marry(&mut data.character_manager, parent_id0, parent_id1);

        let building_id = build(
            &mut data,
            0,
            0,
            BuildingUsage::house(),
            parent_id0,
            parent_id0,
        );

        relocate_to_house(&mut data, vec![parent_id0, parent_id1], building_id);

        let child_id = birth(&mut data, parent_id0, parent_id1);

        assert_eq!(
            get_building_occupied_by(&data.character_manager, child_id),
            Some(building_id)
        );
        assert(
            get_occupants(&data.building_manager, building_id),
            [parent_id0, parent_id1, child_id],
        );
    }
}
