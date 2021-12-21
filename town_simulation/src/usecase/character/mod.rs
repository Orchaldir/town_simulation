use crate::model::character::relation::{Relation, RelationType};
use crate::model::character::{CharacterId, CharacterMgr};
use std::collections::HashSet;

pub mod relation;

pub fn create_child(
    manager: &mut CharacterMgr,
    father: CharacterId,
    mother: CharacterId,
) -> CharacterId {
    let child_id = manager.create();
    let child_relation = Relation::new(RelationType::Child, child_id);

    add_relation(manager, child_id, [father, mother].into(), child_relation);

    child_id
}

fn add_relation(
    manager: &mut CharacterMgr,
    character_id: CharacterId,
    other_ids: HashSet<CharacterId>,
    relation: Relation,
) {
    for other_id in other_ids {
        manager.get_mut(other_id).unwrap().relations.push(relation);

        let other_type = relation.relation_type().reverse();
        let other_relation = Relation::new(other_type, other_id);

        manager
            .get_mut(character_id)
            .unwrap()
            .relations
            .push(other_relation);
    }
}
