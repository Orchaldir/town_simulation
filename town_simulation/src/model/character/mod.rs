#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CharacterId(usize);

impl CharacterId {
    pub fn new(id: usize) -> Self {
        CharacterId(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    id: CharacterId,
}

impl Character {
    pub fn new(id: usize) -> Self {
        Character {
            id: CharacterId::new(id),
        }
    }

    pub fn id(&self) -> &CharacterId {
        &self.id
    }
}
