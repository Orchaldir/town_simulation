use crate::model::character::relation::character::CharacterRelationType;
use crate::model::character::{CharacterId, CharacterMgr};

pub fn get_relation(
    manager: &CharacterMgr,
    from: CharacterId,
    to: CharacterId,
) -> Option<CharacterRelationType> {
    if let Some(character) = manager.get(from) {
        return character
            .character_relations
            .iter()
            .find(|&relation| *relation.id() == to)
            .map(|relation| *relation.relation_type());
    }

    None
}

pub fn is_relative(manager: &CharacterMgr, id0: CharacterId, id1: CharacterId) -> bool {
    get_relation(manager, id0, id1).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::character::relation::character::family::RelativeType::*;
    use crate::model::character::relation::character::CharacterRelationType::*;
    use crate::usecase::character::birth::birth_with_relations;

    #[test]
    fn test_get_relation() {
        let mut manager = CharacterMgr::default();

        // generation 0
        let grandfather = manager.create();
        let grandmother = manager.create();

        // generation 1
        let father = birth_with_relations(&mut manager, grandfather, grandmother);
        let aunt = birth_with_relations(&mut manager, grandfather, grandmother);
        let mother = manager.create();
        let husband_aunt = manager.create();

        // generation 2
        let character = birth_with_relations(&mut manager, father, mother);
        let sibling = birth_with_relations(&mut manager, father, mother);
        let cousin = birth_with_relations(&mut manager, husband_aunt, aunt);

        assert(
            &manager,
            character,
            grandfather,
            Some(Relative(GrandParent)),
        );
        assert(&manager, character, mother, Some(Relative(Parent)));
        assert(&manager, character, aunt, Some(Relative(Pibling)));
        assert(&manager, character, husband_aunt, None);
        assert(&manager, character, sibling, Some(Relative(Sibling)));
        assert(&manager, character, cousin, Some(Relative(Cousin)));

        assert(&manager, grandmother, character, Some(Relative(GrandChild)));
        assert(&manager, father, character, Some(Relative(Child)));
        assert(&manager, aunt, character, Some(Relative(Nibling)));
    }

    fn assert(
        manager: &CharacterMgr,
        from: CharacterId,
        to: CharacterId,
        result: Option<CharacterRelationType>,
    ) {
        assert_eq!(get_relation(&manager, from, to), result);
        assert_eq!(is_relative(&manager, from, to), result.is_some());
    }
}
