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
}
