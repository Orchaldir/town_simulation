use crate::model::character::relation::RelationType;
use crate::model::character::{CharacterId, CharacterMgr};

pub fn get_parents(manager: &CharacterMgr, character_id: CharacterId) -> Vec<CharacterId> {
    get_direct_relation(manager, character_id, RelationType::Parent)
}

pub fn get_children(manager: &CharacterMgr, character_id: CharacterId) -> Vec<CharacterId> {
    get_direct_relation(manager, character_id, RelationType::Child)
}

fn get_direct_relation(
    manager: &CharacterMgr,
    character_id: CharacterId,
    relation_type: RelationType,
) -> Vec<CharacterId> {
    if let Some(character) = manager.get(character_id) {
        return character
            .relations
            .iter()
            .filter(|&relation| *relation.relation_type() == relation_type)
            .map(|relation| *relation.id())
            .collect();
    }
    Vec::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usecase::character::create_child;

    #[test]
    fn test_add() {
        let mut manager = CharacterMgr::default();

        let grandfather0 = manager.create();
        let grandmother0 = manager.create();
        let grandfather1 = manager.create();
        let grandmother1 = manager.create();

        let father = create_child(&mut manager, vec![grandfather0, grandmother0]);
        let mother = create_child(&mut manager, vec![grandfather1, grandmother1]);

        let character0 = create_child(&mut manager, vec![father, mother]);
        let character1 = create_child(&mut manager, vec![father, mother]);

        assert_eq!(get_children(&manager, grandfather0), vec![father]);
        assert_eq!(get_children(&manager, grandmother0), vec![father]);
        assert_eq!(
            get_parents(&manager, father),
            vec![grandfather0, grandmother0]
        );

        assert_eq!(get_children(&manager, grandfather1), vec![mother]);
        assert_eq!(get_children(&manager, grandmother1), vec![mother]);
        assert_eq!(
            get_parents(&manager, mother),
            vec![grandfather1, grandmother1]
        );

        assert_eq!(get_children(&manager, father), vec![character0, character1]);
        assert_eq!(get_children(&manager, mother), vec![character0, character1]);
        assert_eq!(get_parents(&manager, character0), vec![father, mother]);
        assert_eq!(get_parents(&manager, character1), vec![father, mother]);
    }
}
