use crate::model::building::usage::BuildingUsage;
use crate::model::building::{BuildingId, BuildingMgr};
use crate::model::character::relation::building::BuildingRelation;
use crate::model::character::relation::building::BuildingRelationType::{Builder, Owner};
use crate::model::character::{CharacterId, CharacterMgr};
use crate::SimulationData;
use std::collections::HashSet;

pub fn build(
    data: &mut SimulationData,
    block: usize,
    lot: usize,
    usage: BuildingUsage,
    builder: CharacterId,
    owner: CharacterId,
) -> BuildingId {
    let building_id = data
        .building_manager
        .create(usage, data.date, builder, owner);

    data.map.add_building(building_id, block, lot);

    let builder_relation = BuildingRelation::new(Builder, building_id);
    data.character_manager
        .get_mut(builder)
        .unwrap()
        .get_building_relations_mut()
        .push(builder_relation);

    let owner_relation = BuildingRelation::new(Owner, building_id);
    data.character_manager
        .get_mut(owner)
        .unwrap()
        .get_building_relations_mut()
        .push(owner_relation);

    building_id
}

pub fn get_builder(manager: &BuildingMgr, id: BuildingId) -> CharacterId {
    *manager.get(id).unwrap().builder()
}

pub fn get_buildings_build_by(manager: &CharacterMgr, id: CharacterId) -> HashSet<BuildingId> {
    manager
        .get(id)
        .unwrap()
        .building_relations()
        .iter()
        .filter(|&relation| *relation.relation_type() == Builder)
        .map(|relation| *relation.id())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usecase::building::occupancy::get_occupants;
    use crate::usecase::building::ownership::{get_buildings_owned_by, get_owner};
    use crate::util::assert::assert;

    #[test]
    fn test_relations() {
        let mut data = SimulationData::default();
        let builder = data.character_manager.create();
        let owner = data.character_manager.create();

        let building = build(&mut data, 1, 2, BuildingUsage::house(), builder, owner);

        assert_eq!(get_builder(&data.building_manager, building), builder);
        assert_eq!(get_owner(&data.building_manager, building), owner);

        assert(
            get_buildings_build_by(&data.character_manager, builder),
            [building],
        );
        assert!(get_buildings_build_by(&data.character_manager, owner).is_empty());

        assert(
            get_buildings_owned_by(&data.character_manager, owner),
            [building],
        );
        assert!(get_buildings_owned_by(&data.character_manager, builder).is_empty());
    }

    #[test]
    fn no_occupants_after_build() {
        let mut data = SimulationData::default();
        let builder = data.character_manager.create();
        let owner = data.character_manager.create();

        let building = build(&mut data, 1, 2, BuildingUsage::house(), builder, owner);

        assert!(get_occupants(&data.building_manager, building).is_empty());
    }

    #[test]
    fn building_is_added_to_town() {
        let mut data = SimulationData::default();
        let builder = data.character_manager.create();
        let owner = data.character_manager.create();

        let building = build(&mut data, 1, 2, BuildingUsage::house(), builder, owner);

        assert_eq!(data.map.get_building(1, 2), Some(building));
    }
}
