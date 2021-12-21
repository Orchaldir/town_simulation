use crate::model::character::relation::RelationType;
use crate::model::character::{CharacterId, CharacterMgr};
use std::collections::HashSet;

pub fn combine<F>(character_ids: &HashSet<CharacterId>, mut f: F) -> HashSet<CharacterId>
where
    F: FnMut(CharacterId) -> HashSet<CharacterId>,
{
    let mut combined_ids = HashSet::new();

    for character_id in character_ids {
        combined_ids.extend(f(*character_id));
    }

    combined_ids
}

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
    get_direct_relation(manager, character_id, RelationType::Cousin)
}

pub fn get_niblings(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    get_direct_relation(manager, character_id, RelationType::Nibling)
}

pub fn get_grandchildren(
    manager: &CharacterMgr,
    character_id: CharacterId,
) -> HashSet<CharacterId> {
    get_direct_relation(manager, character_id, RelationType::GrandChild)
}

pub fn get_grandparents(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    get_direct_relation(manager, character_id, RelationType::GrandParent)
}

pub fn get_parents(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    get_direct_relation(manager, character_id, RelationType::Parent)
}

pub fn get_piblings(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    get_direct_relation(manager, character_id, RelationType::Pibling)
}

pub fn get_siblings(manager: &CharacterMgr, character_id: CharacterId) -> HashSet<CharacterId> {
    get_direct_relation(manager, character_id, RelationType::Sibling)
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
    fn test_get_relatives() {
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

        // grandchildren of generation 0
        assert(
            get_grandchildren(&manager, grandfather0),
            [character0, character1, character2, cousin],
        );
        assert(
            get_grandchildren(&manager, grandmother0),
            [character0, character1, character2, cousin],
        );
        assert(
            get_grandchildren(&manager, grandfather1),
            [character0, character1, character2],
        );
        assert(
            get_grandchildren(&manager, grandmother1),
            [character0, character1, character2],
        );

        // children of generation 1
        assert(
            get_children(&manager, father),
            [character0, character1, character2],
        );
        assert(
            get_children(&manager, mother),
            [character0, character1, character2],
        );
        assert(get_children(&manager, husband_aunt), [cousin]);
        assert(get_children(&manager, aunt), [cousin]);
        assert(get_children(&manager, uncle), []);

        assert(
            get_shared_children(&manager, father, mother),
            [character0, character1, character2],
        );
        assert(get_shared_children(&manager, husband_aunt, aunt), [cousin]);

        // niblings of generation 1
        assert(get_niblings(&manager, father), [cousin]);
        assert(get_niblings(&manager, mother), []);
        assert(
            get_niblings(&manager, aunt),
            [character0, character1, character2],
        );
        assert(
            get_niblings(&manager, uncle),
            [character0, character1, character2],
        );
        assert(get_niblings(&manager, husband_aunt), []);

        // parents of generation 2
        assert(get_parents(&manager, character0), [father, mother]);
        assert(get_parents(&manager, character1), [father, mother]);
        assert(get_parents(&manager, character2), [father, mother]);
        assert(get_parents(&manager, cousin), [husband_aunt, aunt]);

        // siblings of generation 2
        assert(get_siblings(&manager, character0), [character1, character2]);
        assert(get_siblings(&manager, character1), [character0, character2]);
        assert(get_siblings(&manager, character2), [character0, character1]);
        assert(get_siblings(&manager, cousin), []);

        // piblings of generation 2
        assert(get_piblings(&manager, character0), [aunt, uncle]);
        assert(get_piblings(&manager, character1), [aunt, uncle]);
        assert(get_piblings(&manager, character2), [aunt, uncle]);
        assert(get_piblings(&manager, cousin), [father]);

        // cousins of generation 2
        assert(get_cousins(&manager, character0), [cousin]);
        assert(get_cousins(&manager, character1), [cousin]);
        assert(get_cousins(&manager, character2), [cousin]);
        assert(
            get_cousins(&manager, cousin),
            [character0, character1, character2],
        );

        // grandparents of generation 2
        assert(
            get_grandparents(&manager, character0),
            [grandfather0, grandmother0, grandfather1, grandmother1],
        );
        assert(
            get_grandparents(&manager, character1),
            [grandfather0, grandmother0, grandfather1, grandmother1],
        );
        assert(
            get_grandparents(&manager, character2),
            [grandfather0, grandmother0, grandfather1, grandmother1],
        );
        assert(
            get_grandparents(&manager, cousin),
            [grandfather0, grandmother0],
        );
    }

    fn assert<const N: usize>(left: HashSet<CharacterId>, right: [CharacterId; N]) {
        assert_eq!(left, right.into());
    }
}
