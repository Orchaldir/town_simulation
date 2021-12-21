use crate::model::character::relation::RelationType;
use crate::model::character::{CharacterId, CharacterMgr};
use std::collections::HashSet;

pub fn get_children(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    get_direct_relation(manager, character_id, RelationType::Child)
}

pub fn get_shared_children(
    manager: &CharacterMgr,
    character0: CharacterId,
    character1: CharacterId,
) -> HashSet<CharacterId> {
    get_children(manager, character0)
        .intersection(&get_children(manager, character1))
        .copied()
        .collect()
}

pub fn get_cousins(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    let mut cousins = HashSet::new();

    for pibling_id in get_piblings(manager, character_id) {
        cousins.extend(&get_children(manager, pibling_id));
    }

    cousins
}

pub fn get_niblings(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    let mut niblings = HashSet::new();

    for sibling_id in get_siblings(manager, character_id) {
        niblings.extend(&get_children(manager, sibling_id));
    }

    niblings
}

pub fn get_grandchildren(
    manager: &CharacterMgr,
    character_id: CharacterId,
) -> HashSet<CharacterId> {
    let mut grandchildren = HashSet::new();

    for child_id in get_children(manager, character_id) {
        grandchildren.extend(&get_children(manager, child_id));
    }

    grandchildren
}

pub fn get_grandparents(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    let mut grandparents = HashSet::new();

    for parent_id in get_parents(manager, character_id) {
        grandparents.extend(&get_parents(manager, parent_id));
    }

    grandparents
}

pub fn get_parents(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    get_direct_relation(manager, character_id, RelationType::Parent)
}

pub fn get_piblings(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    let mut piblings = HashSet::new();

    for parent_id in get_parents(manager, character_id) {
        piblings.extend(&get_siblings(manager, parent_id));
    }

    piblings
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

        // generation 0
        let grandfather0 = manager.create();
        let grandmother0 = manager.create();
        let grandfather1 = manager.create();
        let grandmother1 = manager.create();

        // generation 1
        let father = create_child(&mut manager, grandfather0, grandmother0);
        let aunt = create_child(&mut manager, grandfather0, grandmother0);
        let mother = create_child(&mut manager, grandfather1, grandmother1);
        let uncle = create_child(&mut manager, grandfather1, grandmother1);
        let husband_aunt = manager.create();

        // generation 2
        let character0 = create_child(&mut manager, father, mother);
        let character1 = create_child(&mut manager, father, mother);
        let character2 = create_child(&mut manager, father, mother);
        let cousin = create_child(&mut manager, husband_aunt, aunt);

        // children of generation 0
        assert_children(&manager, grandfather0, [aunt, father]);
        assert_children(&manager, grandmother0, [aunt, father]);
        assert_children(&manager, grandfather1, [mother, uncle]);
        assert_children(&manager, grandmother1, [mother, uncle]);

        assert(
            get_shared_children(&manager, grandfather0, grandmother0),
            [aunt, father],
        );
        assert(
            get_shared_children(&manager, grandfather1, grandmother1),
            [mother, uncle],
        );

        // grandchildren of generation 0
        assert_grandchildren(
            &manager,
            grandfather0,
            [character0, character1, character2, cousin],
        );
        assert_grandchildren(
            &manager,
            grandmother0,
            [character0, character1, character2, cousin],
        );
        assert_grandchildren(&manager, grandfather1, [character0, character1, character2]);
        assert_grandchildren(&manager, grandmother1, [character0, character1, character2]);

        // parents of generation 1
        assert_parents(&manager, father, [grandfather0, grandmother0]);
        assert_parents(&manager, aunt, [grandfather0, grandmother0]);
        assert_parents(&manager, mother, [grandfather1, grandmother1]);
        assert_parents(&manager, husband_aunt, []);

        // children of generation 1
        assert_children(&manager, father, [character0, character1, character2]);
        assert_children(&manager, mother, [character0, character1, character2]);
        assert_children(&manager, husband_aunt, [cousin]);
        assert_children(&manager, aunt, [cousin]);
        assert_children(&manager, uncle, []);

        assert(
            get_shared_children(&manager, father, mother),
            [character0, character1, character2],
        );
        assert(get_shared_children(&manager, husband_aunt, aunt), [cousin]);

        // siblings of generation 1
        assert_siblings(&manager, father, [aunt]);
        assert_siblings(&manager, aunt, [father]);
        assert_siblings(&manager, mother, [uncle]);
        assert_siblings(&manager, uncle, [mother]);
        assert_siblings(&manager, husband_aunt, []);

        // niblings of generation 1
        assert_niblings(&manager, father, [cousin]);
        assert_niblings(&manager, mother, []);
        assert_niblings(&manager, aunt, [character0, character1, character2]);
        assert_niblings(&manager, uncle, [character0, character1, character2]);
        assert_niblings(&manager, husband_aunt, []);

        // parents of generation 2
        assert_parents(&manager, character0, [father, mother]);
        assert_parents(&manager, character1, [father, mother]);
        assert_parents(&manager, character2, [father, mother]);
        assert_parents(&manager, cousin, [husband_aunt, aunt]);

        // siblings of generation 2
        assert_siblings(&manager, character0, [character1, character2]);
        assert_siblings(&manager, character1, [character0, character2]);
        assert_siblings(&manager, character2, [character0, character1]);
        assert_siblings(&manager, cousin, []);

        // piblings of generation 2
        assert_piblings(&manager, character0, [aunt, uncle]);
        assert_piblings(&manager, character1, [aunt, uncle]);
        assert_piblings(&manager, character2, [aunt, uncle]);
        assert_piblings(&manager, cousin, [father]);

        // cousins of generation 2
        assert_cousins(&manager, character0, [cousin]);
        assert_cousins(&manager, character1, [cousin]);
        assert_cousins(&manager, character2, [cousin]);
        assert_cousins(&manager, cousin, [character0, character1, character2]);

        // grandparents of generation 2
        assert_grandparents(
            &manager,
            character0,
            [grandfather0, grandmother0, grandfather1, grandmother1],
        );
        assert_grandparents(
            &manager,
            character1,
            [grandfather0, grandmother0, grandfather1, grandmother1],
        );
        assert_grandparents(
            &manager,
            character2,
            [grandfather0, grandmother0, grandfather1, grandmother1],
        );
        assert_grandparents(&manager, cousin, [grandfather0, grandmother0]);
    }

    fn assert<const N: usize>(left: HashSet<CharacterId>, right: [CharacterId; N]) {
        assert_eq!(left, right.into());
    }

    fn assert_children<const N: usize>(
        manager: &CharacterMgr,
        character: CharacterId,
        children: [CharacterId; N],
    ) {
        assert_eq!(get_children(&manager, character), children.into());
    }

    fn assert_grandchildren<const N: usize>(
        manager: &CharacterMgr,
        character: CharacterId,
        grandchildren: [CharacterId; N],
    ) {
        assert_eq!(get_grandchildren(&manager, character), grandchildren.into());
    }

    fn assert_grandparents<const N: usize>(
        manager: &CharacterMgr,
        character: CharacterId,
        grandparents: [CharacterId; N],
    ) {
        assert_eq!(get_grandparents(&manager, character), grandparents.into());
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

    fn assert_piblings<const N: usize>(
        manager: &CharacterMgr,
        character: CharacterId,
        piblings: [CharacterId; N],
    ) {
        assert_eq!(get_piblings(&manager, character), piblings.into());
    }

    fn assert_niblings<const N: usize>(
        manager: &CharacterMgr,
        character: CharacterId,
        niblings: [CharacterId; N],
    ) {
        assert_eq!(get_niblings(&manager, character), niblings.into());
    }

    fn assert_cousins<const N: usize>(
        manager: &CharacterMgr,
        character: CharacterId,
        cousins: [CharacterId; N],
    ) {
        assert_eq!(get_cousins(&manager, character), cousins.into());
    }
}
