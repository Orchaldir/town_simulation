use crate::model::character::{CharacterId, CharacterMgr};
use crate::model::time::Date;

pub fn death(manager: &mut CharacterMgr, id: CharacterId, date: Date) {
    manager.get_mut(id).unwrap().set_death_date(date)
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
        let mut manager = CharacterMgr::default();

        let id = manager.create();

        death(&mut manager, id, Date::new(42));

        assert!(!is_alive(&manager, id));
        assert!(is_dead(&manager, id));
        assert_eq!(get_death_date(&manager, id), &Some(Date::new(42)))
    }

    #[test]
    #[should_panic]
    fn characters_cant_die_twice() {
        let mut manager = CharacterMgr::default();

        let id = manager.create();

        death(&mut manager, id, Date::new(42));
        death(&mut manager, id, Date::new(43));
    }
}
