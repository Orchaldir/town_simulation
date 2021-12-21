use crate::model::character::relation::RelationType;
use crate::model::character::{CharacterId, CharacterMgr};

pub fn get_relation(
    manager: &CharacterMgr,
    from: CharacterId,
    to: CharacterId,
) -> Option<RelationType> {
    if let Some(character) = manager.get(from) {
        return character
            .relations
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
    use crate::model::character::relation::RelationType::*;
    use crate::usecase::character::create_child;

    #[test]
    fn test_get_relation() {
        let mut manager = CharacterMgr::default();

        // generation 0
        let grandfather = manager.create();
        let grandmother = manager.create();

        // generation 1
        let father = create_child(&mut manager, grandfather, grandmother);
        let aunt = create_child(&mut manager, grandfather, grandmother);
        let mother = manager.create();
        let husband_aunt = manager.create();

        // generation 2
        let character = create_child(&mut manager, father, mother);
        let sibling = create_child(&mut manager, father, mother);
        let cousin = create_child(&mut manager, husband_aunt, aunt);

        assert(&manager, character, grandfather, Some(GrandParent));
        assert(&manager, character, mother, Some(Parent));
        assert(&manager, character, aunt, Some(Pibling));
        assert(&manager, character, husband_aunt, None);
        assert(&manager, character, sibling, Some(Sibling));
        assert(&manager, character, cousin, Some(Cousin));

        assert(&manager, grandmother, character, Some(GrandChild));
        assert(&manager, father, character, Some(Child));
        assert(&manager, aunt, character, Some(Nibling));
    }

    fn assert(
        manager: &CharacterMgr,
        from: CharacterId,
        to: CharacterId,
        result: Option<RelationType>,
    ) {
        assert_eq!(get_relation(&manager, from, to), result);
        assert_eq!(is_relative(&manager, from, to), result.is_some());
    }
}
