use crate::model::character::relation::RelationType;
use crate::model::character::{CharacterId, CharacterMgr};
use std::collections::HashSet;

pub fn get_children(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    get_direct_relation(manager, character_id, RelationType::Child)
}

pub fn get_parents(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    get_direct_relation(manager, character_id, RelationType::Parent)
}

pub fn get_siblings(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    let mut siblings = HashSet::new();

    for parent_id in get_parents(manager, character_id) {
        siblings.extend(&get_children(manager, parent_id));
    }

    siblings.remove(&character_id);

    siblings
}

fn get_direct_relation(
    manager: &CharacterMgr,
    character_id: CharacterId,
    relation_type: RelationType,
) -> HashSet<CharacterId> {
    if let Some(character) = manager.get(character_id) {
        return character
            .relations
            .iter()
            .filter(|&relation| *relation.relation_type() == relation_type)
            .map(|relation| *relation.id())
            .collect();
    }
    HashSet::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usecase::character::create_child;

    #[test]
    fn test_relations() {
        let mut manager = CharacterMgr::default();

        let grandfather0 = manager.create();
        let grandmother0 = manager.create();
        let grandfather1 = manager.create();
        let grandmother1 = manager.create();

        let father = create_child(&mut manager, grandfather0, grandmother0);
        let mother = create_child(&mut manager, grandfather1, grandmother1);

        let character0 = create_child(&mut manager, father, mother);
        let character1 = create_child(&mut manager, father, mother);
        let character2 = create_child(&mut manager, father, mother);

        assert_children(&manager, grandfather0, [father]);
        assert_children(&manager, grandmother0, [father]);
        assert_parents(&manager, father, [grandfather0, grandmother0]);

        assert_children(&manager, grandfather1, [mother]);
        assert_children(&manager, grandmother1, [mother]);
        assert_parents(&manager, mother, [grandfather1, grandmother1]);

        assert_children(&manager, father, [character0, character1, character2]);
        assert_children(&manager, mother, [character0, character1, character2]);

        assert_parents(&manager, character0, [father, mother]);
        assert_parents(&manager, character1, [father, mother]);
        assert_parents(&manager, character2, [father, mother]);

        assert_siblings(&manager, character0, [character1, character2]);
        assert_siblings(&manager, character1, [character0, character2]);
        assert_siblings(&manager, character2, [character0, character1]);
    }

    fn assert_children<const N: usize>(
        manager: &CharacterMgr,
        character: CharacterId,
        children: [CharacterId; N],
    ) {
        assert_eq!(get_children(&manager, character), children.into());
    }

    fn assert_parents<const N: usize>(
        manager: &CharacterMgr,
        character: CharacterId,
        parents: [CharacterId; N],
    ) {
        assert_eq!(get_parents(&manager, character), parents.into());
    }

    fn assert_siblings<const N: usize>(
        manager: &CharacterMgr,
        character: CharacterId,
        siblings: [CharacterId; N],
    ) {
        assert_eq!(get_siblings(&manager, character), siblings.into());
    }
}
