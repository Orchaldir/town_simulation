use crate::model::character::relation::Relation;
use crate::model::character::relation::RelationType::Spouse;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::usecase::character::add_relation;
use crate::usecase::character::relation::get::get_relation_of_relatives;

pub fn marry(manager: &mut CharacterMgr, id0: CharacterId, id1: CharacterId) {
    update_in_laws(manager, id0, id1);
    update_in_laws(manager, id1, id0);
    add_relation(manager, id0, &vec![id1].into_iter().collect(), Spouse);
}

fn update_in_laws(manager: &mut CharacterMgr, from: CharacterId, to: CharacterId) {
    let in_laws: Vec<Relation> = get_relation_of_relatives(manager, from)
        .iter()
        .map(|&relation| relation.to_in_law())
        .flatten()
        .collect();

    for in_law in in_laws {
        manager.get_mut(to).unwrap().relations.push(in_law);
    }
}
