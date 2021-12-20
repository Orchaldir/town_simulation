use crate::model::character::relation::RelationType;
use crate::model::character::{CharacterId, CharacterMgr};

pub fn get_parents(manager: &CharacterMgr, character_id: CharacterId) -> Vec<CharacterId> {
    if let Some(character) = manager.get(character_id) {
        return character
            .relations
            .iter()
            .filter(|&relation| *relation.relation_type() == RelationType::Parent)
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

        assert_eq!(
            get_parents(&manager, father),
            vec![grandfather0, grandmother0]
        );
        assert_eq!(
            get_parents(&manager, mother),
            vec![grandfather1, grandmother1]
        );
    }
}
