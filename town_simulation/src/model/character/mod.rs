use crate::model::character::gender::Gender;
use crate::model::character::name::CharacterName;
use crate::model::character::relation::Relation;
use crate::model::time::Date;
use derive_more::Constructor;

pub mod gender;
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
    gender: Gender,
    birth_date: Date,
    death_date: Option<Date>,
    pub relations: Vec<Relation>,
}

impl Character {
    pub fn new(id: CharacterId) -> Self {
        Character {
            id,
            name: CharacterName::simple(id.0.to_string()),
            gender: Gender::default(),
            birth_date: Date::default(),
            death_date: None,
            relations: Vec::new(),
        }
    }

    pub fn id(&self) -> &CharacterId {
        &self.id
    }

    pub fn name(&self) -> &CharacterName {
        &self.name
    }

    pub fn set_name(&mut self, name: CharacterName) {
        self.name = name;
    }

    pub fn gender(&self) -> &Gender {
        &self.gender
    }

    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }

    pub fn birth_date(&self) -> &Date {
        &self.birth_date
    }

    pub fn get_age(&self, date: Date) -> u32 {
        date.get_year() - self.birth_date.get_year()
    }

    pub fn set_birth_date(&mut self, birth_date: Date) {
        self.birth_date = birth_date;
    }

    pub fn is_alive(&self) -> bool {
        self.death_date.is_none()
    }

    pub fn is_dead(&self) -> bool {
        self.death_date.is_some()
    }

    pub fn death_date(&self) -> &Option<Date> {
        &self.death_date
    }

    pub fn set_death_date(&mut self, death_date: Date) {
        if self.is_dead() {
            panic!("Character is already dead!");
        }

        self.death_date = Some(death_date);
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
