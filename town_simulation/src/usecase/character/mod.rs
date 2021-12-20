use crate::model::character::relation::{Relation, RelationType};
use crate::model::character::{CharacterId, CharacterMgr};

pub mod relation;

pub fn create_child(
    manager: &mut CharacterMgr,
    father: CharacterId,
    mother: CharacterId,
) -> CharacterId {
    create_child_with_many_parents(manager, vec![father, mother])
}

pub fn create_child_with_many_parents(
    manager: &mut CharacterMgr,
    parent_ids: Vec<CharacterId>,
) -> CharacterId {
    let child_id = manager.create();
    let child_relation = Relation::new(RelationType::Child, child_id);

    for parent_id in parent_ids {
        manager
            .get_mut(parent_id)
            .unwrap()
            .relations
            .push(child_relation);

        let parent_relation = Relation::new(RelationType::Parent, parent_id);
        manager
            .get_mut(child_id)
            .unwrap()
            .relations
            .push(parent_relation);
    }

    child_id
}
