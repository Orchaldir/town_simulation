use crate::model::character::relation::Relation;
use derive_more::Constructor;

pub mod relation;

#[derive(Constructor, Copy, Clone, Debug, PartialEq)]
pub struct CharacterId(usize);

impl CharacterId {
    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    id: CharacterId,
    relations: Vec<Relation>,
}

impl Character {
    pub fn new(id: usize) -> Self {
        Character {
            id: CharacterId::new(id),
            relations: Vec::new(),
        }
    }

    pub fn id(&self) -> &CharacterId {
        &self.id
    }
}
