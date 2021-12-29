use crate::model::building::usage::BuildingUsage;
use crate::model::building::BuildingId;
use crate::model::character::CharacterId;
use crate::usecase::building::build::build;
use crate::usecase::building::relocate::relocate_to_house;
use crate::SimulationData;

pub fn relocate(data: &mut SimulationData, character_ids: Vec<CharacterId>) {
    let building_id = find_or_build_new_home(data, &character_ids);

    relocate_to_house(data, character_ids, building_id);
}

fn find_or_build_new_home(data: &mut SimulationData, character_ids: &[CharacterId]) -> BuildingId {
    if let Some(building_id) = find_empty_home(data) {
        return building_id;
    }

    let (block, lot) = find_best_location(data);
    let builder = character_ids[0];

    build(data, block, lot, BuildingUsage::house(), builder, builder)
}

fn find_empty_home(data: &SimulationData) -> Option<BuildingId> {
    for building in data.building_manager.get_all() {
        match building.usage() {
            BuildingUsage::Apartments(homes) => {
                if homes.iter().any(|home| home.is_empty()) {
                    return Some(*building.id());
                }
            }
            BuildingUsage::House(home) => {
                if home.is_empty() {
                    return Some(*building.id());
                }
            }
        }
    }

    None
}

fn find_best_location(data: &SimulationData) -> (usize, usize) {
    for block in 0..data.map.blocks().len() {
        for lot in 0..4 {
            if data.map.is_lot_free(block, lot) {
                return (block, lot);
            }
        }
    }

    (0, 0)
}
