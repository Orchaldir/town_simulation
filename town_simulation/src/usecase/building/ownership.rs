use crate::model::building::{BuildingId, BuildingMgr};
use crate::model::character::relation::building::BuildingRelation;
use crate::model::character::relation::building::BuildingRelationType::Owner;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::usecase::building::get_building_relation;
use crate::SimulationData;
use std::collections::HashSet;

pub fn get_owner(manager: &BuildingMgr, id: BuildingId) -> CharacterId {
    *manager.get(id).unwrap().owner()
}

pub fn get_buildings_owned_by(manager: &CharacterMgr, id: CharacterId) -> HashSet<BuildingId> {
    get_building_relation(manager, id, Owner)
}

pub fn update_owner(
    data: &mut SimulationData,
    building_id: BuildingId,
    owner_id: CharacterId,
    new_owner_id: CharacterId,
) {
    data.building_manager
        .get_mut(building_id)
        .unwrap()
        .update_owner(new_owner_id);

    data.character_manager
        .get_mut(owner_id)
        .unwrap()
        .remove_ownership(building_id);

    add_ownership(&mut data.character_manager, building_id, new_owner_id);
}

pub fn add_ownership(manager: &mut CharacterMgr, building_id: BuildingId, owner_id: CharacterId) {
    let owner_relation = BuildingRelation::new(Owner, building_id);
    manager
        .get_mut(owner_id)
        .unwrap()
        .get_building_relations_mut()
        .push(owner_relation);
}
