use crate::model::character::relation::Relation;
use crate::model::character::relation::RelationType::Spouse;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::usecase::character::relation::get::get_relation_to_relatives;
use crate::usecase::character::{add_relation, add_relations};

pub fn marry(manager: &mut CharacterMgr, id0: CharacterId, id1: CharacterId) {
    update_in_laws(manager, id0, id1);
    update_in_laws(manager, id1, id0);
    add_relations(manager, id0, &vec![id1].into_iter().collect(), Spouse);
    update_names(manager, id0, id1);
}

fn update_in_laws(manager: &mut CharacterMgr, from: CharacterId, to: CharacterId) {
    let in_laws: Vec<Relation> = get_relation_to_relatives(manager, from)
        .iter()
        .map(|&relation| relation.to_in_law())
        .flatten()
        .collect();

    for in_law in in_laws {
        add_relation(manager, *in_law.id(), to, *in_law.relation_type());
    }
}

fn update_names(manager: &mut CharacterMgr, id0: CharacterId, id1: CharacterId) {
    if let Some(last_name) = manager.get(id0).map(|c| c.name().get_last()).flatten() {
        let last_name = last_name.to_string();

        if let Some(character) = manager.get_mut(id1) {
            let name = character.name().marry(last_name);
            character.set_name(name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::character::name::CharacterName;
    use crate::model::character::relation::family::RelativeType;
    use crate::model::character::relation::family::RelativeType::{Child, Parent};
    use crate::model::character::relation::RelationType::{InLaw, Relative};
    use crate::usecase::character::relation::get::{get_relation_to_in_laws, get_spouses};
    use crate::usecase::character::{get_name, set_name};
    use std::collections::HashSet;

    #[test]
    fn test_husband_and_wife_are_spouses() {
        let mut manager = CharacterMgr::default();

        let husband = manager.create();
        let wife = manager.create();

        marry(&mut manager, husband, wife);

        assert(get_spouses(&manager, husband), [wife]);
        assert(get_spouses(&manager, wife), [husband]);
    }

    #[test]
    fn test_wife_takes_name_of_husband() {
        let mut manager = CharacterMgr::default();

        let husband = manager.create();
        let wife = manager.create();

        let husband_name = CharacterName::standard("A", "B");
        let wive_name = CharacterName::standard("C", "D");
        let married_name = CharacterName::married("C", "B", "D");

        set_name(&mut manager, husband, husband_name.clone());
        set_name(&mut manager, wife, wive_name);

        marry(&mut manager, husband, wife);

        assert_eq!(get_name(&manager, husband), &husband_name);
        assert_eq!(get_name(&manager, wife), &married_name);
    }

    #[test]
    fn test_update_in_laws() {
        let mut manager = CharacterMgr::default();

        let husband = manager.create();
        let wife = manager.create();

        let husband_parent = manager.create();
        let wife_parent = manager.create();

        add_relation(&mut manager, husband, husband_parent, Relative(Child));
        add_relation(&mut manager, wife, wife_parent, Relative(Child));

        marry(&mut manager, husband, wife);

        assert_in_law(&manager, husband, wife_parent, Parent);
        assert_in_law(&manager, wife, husband_parent, Parent);
    }

    fn assert_in_law(
        manager: &CharacterMgr,
        character: CharacterId,
        in_law: CharacterId,
        in_law_type: RelativeType,
    ) {
        assert_eq!(
            get_relation_to_in_laws(&manager, character),
            vec![&Relation::new(InLaw(in_law_type), in_law)]
        );
        assert_eq!(
            get_relation_to_in_laws(&manager, in_law),
            vec![&Relation::new(InLaw(in_law_type.reverse()), character)]
        );
    }

    fn assert<const N: usize>(left: HashSet<CharacterId>, right: [CharacterId; N]) {
        assert_eq!(left, right.into());
    }
}
