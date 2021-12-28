use crate::model::building::BuildingId;
use crate::model::character::relation::building::BuildingRelationType;
use crate::model::character::{CharacterId, CharacterMgr};
use std::collections::HashSet;

pub mod build;
pub mod occupancy;
pub mod ownership;
pub mod relocate;

pub fn get_building_relation(
    manager: &CharacterMgr,
    id: CharacterId,
    relation_type: BuildingRelationType,
) -> HashSet<BuildingId> {
    manager
        .get(id)
        .unwrap()
        .building_relations()
        .iter()
        .filter(|&relation| *relation.relation_type() == relation_type)
        .map(|relation| *relation.id())
        .collect()
}
