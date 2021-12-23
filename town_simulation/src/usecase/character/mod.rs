use crate::generation::name::character::CharacterNameGenerator;
use crate::model::character::gender::Gender;
use crate::model::character::name::CharacterName;
use crate::model::character::relation::{Relation, RelationType};
use crate::model::character::{CharacterId, CharacterMgr};
use std::collections::HashSet;

pub mod birth;
pub mod marriage;
pub mod relation;

pub fn set_name(manager: &mut CharacterMgr, id: CharacterId, name: CharacterName) {
    let character = manager.get_mut(id).unwrap();
    character.set_name(name);
}

pub fn set_generated_name(
    manager: &mut CharacterMgr,
    generator: &CharacterNameGenerator,
    id: CharacterId,
) {
    set_name(manager, id, generator.generate(manager, id));
}

pub fn set_gender(manager: &mut CharacterMgr, id: CharacterId, gender: Gender) {
    let character = manager.get_mut(id).unwrap();
    character.set_gender(gender);
}

pub fn set_gender_based_on_id(manager: &mut CharacterMgr, id: CharacterId) {
    let gender = if id.id() % 2 == 0 {
        Gender::Male
    } else {
        Gender::Female
    };
    set_gender(manager, id, gender);
}

pub fn add_relation(
    manager: &mut CharacterMgr,
    character: CharacterId,
    others: &HashSet<CharacterId>,
    relation_type: RelationType,
) {
    let relation = Relation::new(relation_type, character);
    let other_type = relation_type.reverse();

    for other in others {
        manager.get_mut(*other).unwrap().relations.push(relation);

        let other_relation = Relation::new(other_type, *other);

        manager
            .get_mut(character)
            .unwrap()
            .relations
            .push(other_relation);
    }
}
