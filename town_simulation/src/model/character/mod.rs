use crate::model::character::relation::Relation;
use derive_more::Constructor;
use crate::model::character::name::CharacterName;

pub mod name;
pub mod relation;

#[derive(Constructor, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CharacterId(usize);

impl CharacterId {
    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    id: CharacterId,
    name: CharacterName,
    pub relations: Vec<Relation>,
}

impl Character {
    pub fn new(id: CharacterId) -> Self {
        Character {
            id,
            name: CharacterName::simple(id.0.to_string()),
            relations: Vec::new(),
        }
    }

    pub fn id(&self) -> &CharacterId {
        &self.id
    }
}

#[derive(Default, Debug)]
pub struct CharacterMgr {
    characters: Vec<Character>,
}

impl CharacterMgr {
    pub fn create(&mut self) -> CharacterId {
        let id = CharacterId::new(self.characters.len());
        self.characters.push(Character::new(id));
        id
    }

    pub fn get_all(&self) -> &Vec<Character> {
        &self.characters
    }

    pub fn get(&self, id: CharacterId) -> Option<&Character> {
        self.characters.get(id.0)
    }

    pub fn get_mut(&mut self, id: CharacterId) -> Option<&mut Character> {
        self.characters.get_mut(id.0)
    }
}
