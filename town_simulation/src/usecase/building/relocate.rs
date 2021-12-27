use crate::model::building::usage::BuildingUsage::House;
use crate::model::building::BuildingId;
use crate::model::character::relation::building::BuildingRelation;
use crate::model::character::relation::building::BuildingRelationType::Occupant;
use crate::model::character::CharacterId;
use crate::SimulationData;

pub fn relocate_to_house(
    data: &mut SimulationData,
    character_ids: Vec<CharacterId>,
    building_id: BuildingId,
) {
    if let House(home) = data
        .building_manager
        .get_mut(building_id)
        .unwrap()
        .get_usage_mut()
    {
        if home.is_empty() {
            home.get_occupants_mut().extend(&character_ids);
        } else {
            panic!("House {} is not empty!", building_id.id());
        }
    } else {
        panic!("Building {} is not a house!", building_id.id());
    }

    for character_id in character_ids {
        let character = data.character_manager.get_mut(character_id).unwrap();

        character
            .get_building_relations_mut()
            .retain(|relation| *relation.relation_type() != Occupant);

        character
            .get_building_relations_mut()
            .push(BuildingRelation::new(Occupant, building_id));
    }
}
