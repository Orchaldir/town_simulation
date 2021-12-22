use crate::model::character::relation::RelationType::Spouse;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::usecase::character::add_relation;

pub fn marry(manager: &mut CharacterMgr, id0: CharacterId, id1: CharacterId) {
    add_relation(manager, id0, &vec![id1].into_iter().collect(), Spouse);
}
