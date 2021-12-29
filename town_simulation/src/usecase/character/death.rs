use crate::model::character::{CharacterId, CharacterMgr};
use crate::model::time::Date;
use crate::usecase::building::occupancy::remove_occupant_from_building;
use crate::usecase::building::ownership::{get_buildings_owned_by, update_owner};
use crate::usecase::character::relation::get::{get_children, get_grandchildren, get_spouses};
use crate::SimulationData;

pub fn death(data: &mut SimulationData, id: CharacterId) {
    remove_occupant_from_building(data, id);

    let character = data.character_manager.get_mut(id).unwrap();

    character.set_death_date(data.date);
    character.remove_occupancy();

    inherit(data, id);
}

pub fn is_alive(manager: &CharacterMgr, id: CharacterId) -> bool {
    manager.get(id).unwrap().is_alive()
}

pub fn is_dead(manager: &CharacterMgr, id: CharacterId) -> bool {
    manager.get(id).unwrap().is_dead()
}

pub fn get_death_date(manager: &CharacterMgr, id: CharacterId) -> &Option<Date> {
    manager.get(id).unwrap().death_date()
}

fn inherit(data: &mut SimulationData, id: CharacterId) {
    if let Some(heir_id) = get_heir(&data.character_manager, id) {
        for building_id in get_buildings_owned_by(&data.character_manager, id) {
            update_owner(data, building_id, id, heir_id);
        }
    }
}

fn get_heir(manager: &CharacterMgr, id: CharacterId) -> Option<CharacterId> {
    for spouse_id in get_spouses(manager, id) {
        if is_alive(manager, spouse_id) {
            return Some(spouse_id);
        }
    }

    for child_id in get_children(manager, id) {
        if is_alive(manager, child_id) {
            return Some(child_id);
        }
    }

    for grandchild_id in get_grandchildren(manager, id) {
        if is_alive(manager, grandchild_id) {
            return Some(grandchild_id);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::building::usage::BuildingUsage;
    use crate::usecase::building::build::{build, get_builder};
    use crate::usecase::building::occupancy::{get_building_occupied_by, get_occupants};
    use crate::usecase::building::ownership::get_owner;
    use crate::usecase::building::relocate::relocate_to_house;
    use crate::usecase::character::marriage::marry;
    use crate::util::assert::assert;

    #[test]
    fn characters_start_alive() {
        let mut manager = CharacterMgr::default();

        let id = manager.create();

        assert!(is_alive(&manager, id));
        assert!(!is_dead(&manager, id));
        assert_eq!(get_death_date(&manager, id), &None)
    }

    #[test]
    fn characters_are_dead_after_death() {
        let mut data = SimulationData::default();

        let id = data.character_manager.create();

        data.date = Date::new(42);

        death(&mut data, id);

        assert!(!is_alive(&data.character_manager, id));
        assert!(is_dead(&data.character_manager, id));
        assert_eq!(
            get_death_date(&data.character_manager, id),
            &Some(Date::new(42))
        )
    }

    #[test]
    #[should_panic]
    fn characters_cant_die_twice() {
        let mut data = SimulationData::default();

        let id = data.character_manager.create();

        data.date = Date::new(42);

        death(&mut data, id);

        data.date = Date::new(43);

        death(&mut data, id);
    }

    #[test]
    fn dead_characters_are_not_occupants() {
        let mut data = SimulationData::default();

        let character_id = data.character_manager.create();
        let other_id = data.character_manager.create();
        let building_id =
            data.building_manager
                .create(BuildingUsage::house(), data.date, other_id, other_id);

        relocate_to_house(&mut data, vec![character_id, other_id], building_id);

        death(&mut data, character_id);

        assert_eq!(
            get_building_occupied_by(&data.character_manager, character_id),
            None
        );
        assert_eq!(
            get_building_occupied_by(&data.character_manager, other_id),
            Some(building_id)
        );
        assert(
            get_occupants(&data.building_manager, building_id),
            [other_id],
        );
    }

    #[test]
    fn child_inherits_after_death() {
        let mut data = SimulationData::default();

        let character_id = data.character_manager.create();
        let spouse_id = data.character_manager.create();

        marry(&mut data.character_manager, character_id, spouse_id);

        let building_id = build(
            &mut data,
            0,
            0,
            BuildingUsage::house(),
            character_id,
            character_id,
        );

        relocate_to_house(&mut data, vec![character_id, spouse_id], building_id);

        death(&mut data, character_id);

        assert_eq!(
            get_builder(&data.building_manager, building_id),
            character_id
        );
        assert_eq!(get_owner(&data.building_manager, building_id), spouse_id);
        assert(
            get_buildings_owned_by(&data.character_manager, character_id),
            [],
        );
        assert(
            get_buildings_owned_by(&data.character_manager, spouse_id),
            [building_id],
        );
    }
}
