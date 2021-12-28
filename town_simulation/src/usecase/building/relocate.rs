use crate::model::building::usage::BuildingUsage::House;
use crate::model::building::BuildingId;
use crate::model::character::CharacterId;
use crate::usecase::building::occupancy::{get_building_occupied_by, remove_occupant_from_building};
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
        remove_occupant_from_building(data, character_id);

        data.character_manager
            .get_mut(character_id)
            .unwrap()
            .relocate(building_id);
    }
}

pub fn join_parents_home(
    data: &mut SimulationData,
    character_ids: Vec<CharacterId>,
    parent_id: CharacterId,
) {
    let building_id = get_building_occupied_by(&data.character_manager, parent_id).unwrap();
    let building = data
        .building_manager
        .get_mut(building_id).unwrap();

    if let House(home) = building
        .get_usage_mut()
    {
        home.get_occupants_mut().extend(&character_ids);
    }

    for character_id in character_ids {
        data.character_manager
            .get_mut(character_id)
            .unwrap()
            .relocate(building_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::building::usage::BuildingUsage;
    use crate::usecase::building::build::build;
    use crate::usecase::building::occupancy::{get_building_occupied_by, get_occupants};
    use crate::util::assert::assert;

    #[test]
    fn test_relocate() {
        let mut data = SimulationData::default();
        let builder = data.character_manager.create();
        let owner = data.character_manager.create();
        let occupant0 = data.character_manager.create();
        let occupant1 = data.character_manager.create();

        let building = build(&mut data, 1, 2, BuildingUsage::house(), builder, owner);

        relocate_to_house(&mut data, vec![occupant0, occupant1], building);

        assert(
            get_occupants(&data.building_manager, building),
            [occupant0, occupant1],
        );

        assert_eq!(
            get_building_occupied_by(&data.character_manager, occupant0),
            Some(building)
        );
        assert_eq!(
            get_building_occupied_by(&data.character_manager, occupant1),
            Some(building)
        );
    }

    #[test]
    fn relocate_to_another_house() {
        let mut data = SimulationData::default();
        let builder = data.character_manager.create();
        let owner = data.character_manager.create();
        let occupant = data.character_manager.create();

        let building0 = build(&mut data, 1, 2, BuildingUsage::house(), builder, owner);
        let building1 = build(&mut data, 1, 3, BuildingUsage::house(), builder, owner);

        relocate_to_house(&mut data, vec![occupant], building0);
        relocate_to_house(&mut data, vec![occupant], building1);

        assert(get_occupants(&data.building_manager, building0), []);
        assert(get_occupants(&data.building_manager, building1), [occupant]);

        assert_eq!(
            get_building_occupied_by(&data.character_manager, occupant),
            Some(building1)
        );
    }
}
