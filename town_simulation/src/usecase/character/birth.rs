use crate::model::character::relation::character::family::RelativeType;
use crate::model::character::relation::character::family::RelativeType::*;
use crate::model::character::relation::character::RelationType::*;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::model::time::Date;
use crate::usecase::character::relation::get::*;
use crate::usecase::character::{add_relation, add_relations};
use std::collections::HashSet;

pub fn birth(manager: &mut CharacterMgr, father: CharacterId, mother: CharacterId) -> CharacterId {
    let parents = [father, mother].into();
    let child = manager.create();
    let siblings = get_shared_children(manager, father, mother);
    let grandparents = combine(&parents, |id| get_parents(manager, id));
    let piblings = combine(&parents, |id| get_siblings(manager, id));
    let cousins = combine(&piblings, |id| get_children(manager, id));

    add_relations(manager, child, &grandparents, Relative(GrandChild));
    add_relations(manager, child, &cousins, Relative(Cousin));
    add_relations(manager, child, &piblings, Relative(Nibling));
    add_relations(manager, child, &siblings, Relative(Sibling));
    add_relations(manager, child, &parents, Relative(Child));

    let mut relatives = HashSet::new();
    relatives.extend(parents.clone());
    relatives.extend(siblings.clone());
    relatives.extend(grandparents.clone());
    relatives.extend(piblings.clone());
    relatives.extend(cousins.clone());

    add_in_laws(manager, child, &grandparents, &relatives, GrandChild);
    add_in_laws(manager, child, &cousins, &relatives, Cousin);
    add_in_laws(manager, child, &piblings, &relatives, Nibling);
    add_in_laws(manager, child, &siblings, &relatives, Sibling);
    add_in_laws(manager, child, &parents, &relatives, Child);

    child
}

pub fn set_birth_date(manager: &mut CharacterMgr, id: CharacterId, date: Date) {
    manager.get_mut(id).unwrap().set_birth_date(date)
}

pub fn get_birth_date(manager: &CharacterMgr, id: CharacterId) -> &Date {
    manager.get(id).unwrap().birth_date()
}

fn add_in_laws(
    manager: &mut CharacterMgr,
    character: CharacterId,
    targets: &HashSet<CharacterId>,
    relatives: &HashSet<CharacterId>,
    relative_type: RelativeType,
) {
    let relation_type = InLaw(relative_type);

    for target in targets {
        for spouse in get_spouses(manager, *target) {
            if !relatives.contains(&spouse) {
                add_relation(manager, character, spouse, relation_type);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_birth_date_of_character() {
        let mut manager = CharacterMgr::default();
        let id = manager.create();
        let date = Date::new(42);

        set_birth_date(&mut manager, id, date);

        assert_eq!(get_birth_date(&manager, id), &date)
    }
}
