use crate::model::character::{Character, CharacterId, CharacterMgr};
use crate::model::character::relation::{Relation, RelationType};

pub fn create_child(manager: &mut CharacterMgr, parent_ids: Vec<CharacterId>) -> CharacterId {
    let child_id = manager.create();
    let child_relation = Relation::new(RelationType::Child, child_id);

    for parent_id in parent_ids {
        manager.get_mut(parent_id).unwrap().relations.push(child_relation);

        let parent_relation = Relation::new(RelationType::Child, parent_id);
        manager.get_mut(child_id).unwrap().relations.push(parent_relation);
    }

    child_id
}
