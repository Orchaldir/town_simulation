use crate::model::character::relation::Relation;
use crate::model::character::relation::RelationType::Spouse;
use crate::model::character::{Character, CharacterId, CharacterMgr};
use crate::usecase::character::relation::get::{get_relation_to_relatives, get_spouses};
use crate::usecase::character::{add_relation, add_relations};
use std::collections::HashSet;

pub fn get_married_couples(manager: &CharacterMgr) -> HashSet<(CharacterId, CharacterId)> {
    let mut couples = HashSet::new();

    for character in manager.get_all() {
        let id = *character.id();

        for spouse in get_spouses(&manager, id) {
            couples.insert(if id.id() < spouse.id() {
                (id, spouse)
            } else {
                (spouse, id)
            });
        }
    }

    couples
}

pub fn get_unmarried(manager: &CharacterMgr) -> HashSet<CharacterId> {
    manager.get_all()
        .iter()
        .filter(|&character| !is_married(character))
        .map(|character| *character.id())
        .collect()
}

pub fn is_married(character: &Character) -> bool {
    character.relations.iter().find(|&relation| *relation.relation_type() == Spouse).is_some()
}

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
    use std::fmt::Debug;
    use std::hash::Hash;

    #[test]
    fn husband_and_wife_are_spouses() {
        let mut manager = CharacterMgr::default();

        let husband = manager.create();
        let wife = manager.create();

        marry(&mut manager, husband, wife);

        assert(get_spouses(&manager, husband), [wife]);
        assert(get_spouses(&manager, wife), [husband]);
    }

    #[test]
    fn husband_and_wife_are_couples() {
        let mut manager = CharacterMgr::default();

        let husband0 = manager.create();
        let wife0 = manager.create();
        let wife1 = manager.create();
        let husband1 = manager.create();
        manager.create();

        marry(&mut manager, husband0, wife0);
        marry(&mut manager, husband1, wife1);

        assert(
            get_married_couples(&manager),
            [(husband0, wife0), (wife1, husband1)],
        );
    }

    #[test]
    fn not_married_characters_are_unmarried() {
        let mut manager = CharacterMgr::default();

        let husband = manager.create();
        let wife = manager.create();
        let character = manager.create();

        assert(get_unmarried(&manager), [husband, wife, character]);

        marry(&mut manager, husband, wife);

        assert(get_unmarried(&manager), [character]);
    }

    #[test]
    fn wife_takes_name_of_husband() {
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
    fn relatives_of_spouse_become_in_laws() {
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

    fn assert<T: Eq + Hash + Debug, const N: usize>(left: HashSet<T>, right: [T; N]) {
        assert_eq!(left, right.into());
    }
}
