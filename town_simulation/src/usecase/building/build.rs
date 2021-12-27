use crate::model::building::usage::BuildingUsage;
use crate::model::building::BuildingId;
use crate::model::character::relation::building::BuildingRelation;
use crate::model::character::relation::building::BuildingRelationType::{Builder, Owner};
use crate::model::character::CharacterId;
use crate::SimulationData;

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
