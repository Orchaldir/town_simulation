use crate::model::building::{BuildingId, BuildingMgr};
use crate::model::character::relation::building::BuildingRelationType::Owner;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::usecase::building::get_building_relation;
use std::collections::HashSet;

pub fn get_owner(manager: &BuildingMgr, id: BuildingId) -> CharacterId {
    *manager.get(id).unwrap().owner()
}

pub fn get_buildings_owned_by(manager: &CharacterMgr, id: CharacterId) -> HashSet<BuildingId> {
    get_building_relation(manager, id, Owner)
}
