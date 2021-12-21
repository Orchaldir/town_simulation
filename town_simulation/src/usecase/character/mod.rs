use crate::model::character::relation::{Relation, RelationType};
use crate::model::character::{CharacterId, CharacterMgr};
use crate::usecase::character::relation::get::{
    combine, get_children, get_parents, get_shared_children, get_siblings,
};
use std::collections::HashSet;
use RelationType::*;

pub mod relation;

pub fn create_child(
    manager: &mut CharacterMgr,
    father: CharacterId,
    mother: CharacterId,
) -> CharacterId {
    let parents = [father, mother].into();
    let child_id = manager.create();
    let siblings = get_shared_children(manager, father, mother);
    let grandparents = combine(&parents, |id| get_parents(manager, id));
    let piblings = combine(&parents, |id| get_siblings(manager, id));
    let cousins = combine(&piblings, |id| get_children(manager, id));

    add_relation(manager, child_id, &grandparents, GrandChild);
    add_relation(manager, child_id, &cousins, Cousin);
    add_relation(manager, child_id, &piblings, Nibling);
    add_relation(manager, child_id, &siblings, Sibling);
    add_relation(manager, child_id, &parents, Child);

    child_id
}

fn add_relation(
    manager: &mut CharacterMgr,
    character_id: CharacterId,
    other_ids: &HashSet<CharacterId>,
    relation_type: RelationType,
) {
    let relation = Relation::new(relation_type, character_id);
    let other_type = relation_type.reverse();

    for other_id in other_ids {
        manager.get_mut(*other_id).unwrap().relations.push(relation);

        let other_relation = Relation::new(other_type, *other_id);

        manager
            .get_mut(character_id)
            .unwrap()
            .relations
            .push(other_relation);
    }
}
