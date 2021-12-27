use crate::model::building::{BuildingId, BuildingMgr};
use crate::model::character::relation::building::BuildingRelationType::Owner;
use crate::model::character::{CharacterId, CharacterMgr};
use std::collections::HashSet;

pub fn get_owner(manager: &BuildingMgr, id: BuildingId) -> CharacterId {
    *manager.get(id).unwrap().owner()
}

pub fn get_buildings_owned_by(manager: &CharacterMgr, id: CharacterId) -> HashSet<BuildingId> {
    manager
        .get(id)
        .unwrap()
        .building_relations()
        .iter()
        .filter(|&relation| *relation.relation_type() == Owner)
        .map(|relation| *relation.id())
        .collect()
}
