use crate::model::building::usage::BuildingUsage;
use crate::model::building::{BuildingId, BuildingMgr};
use crate::model::character::relation::building::BuildingRelationType::Occupant;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::SimulationData;
use std::collections::HashSet;

pub fn get_occupants(manager: &BuildingMgr, id: BuildingId) -> HashSet<CharacterId> {
    match manager.get(id).unwrap().usage() {
        BuildingUsage::Apartments(homes) => homes
            .iter()
            .flat_map(|home| home.occupants().clone())
            .into_iter()
            .collect(),
        BuildingUsage::House(home) => home.occupants().clone().into_iter().collect(),
    }
}

pub fn get_building_occupied_by(manager: &CharacterMgr, id: CharacterId) -> Option<BuildingId> {
    manager
        .get(id)
        .unwrap()
        .building_relations()
        .iter()
        .filter(|&relation| *relation.relation_type() == Occupant)
        .map(|relation| *relation.id())
        .next()
}

pub fn remove_occupant_from_building(data: &mut SimulationData, character_id: CharacterId) {
    if let Some(building_id) = get_building_occupied_by(&data.character_manager, character_id) {
        data.building_manager
            .get_mut(building_id)
            .unwrap()
            .remove_occupant(character_id);
    }
}
