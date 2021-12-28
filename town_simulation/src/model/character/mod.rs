use crate::model::building::BuildingId;
use crate::model::character::gender::Gender;
use crate::model::character::name::CharacterName;
use crate::model::character::relation::building::BuildingRelation;
use crate::model::character::relation::building::BuildingRelationType::Occupant;
use crate::model::character::relation::character::CharacterRelation;
use crate::model::time::Date;
use derive_getters::Getters;
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

#[derive(Getters, Clone, Debug, PartialEq)]
pub struct Character {
    id: CharacterId,
    name: CharacterName,
    gender: Gender,
    birth_date: Date,
    death_date: Option<Date>,
    pub character_relations: Vec<CharacterRelation>,
    building_relations: Vec<BuildingRelation>,
}

impl Character {
    pub fn new(id: CharacterId) -> Self {
        Character {
            id,
            name: CharacterName::simple(id.0.to_string()),
            gender: Gender::default(),
            birth_date: Date::default(),
            death_date: None,
            character_relations: Vec::new(),
            building_relations: Vec::new(),
        }
    }

    pub fn set_name(&mut self, name: CharacterName) {
        self.name = name;
    }

    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }

    pub fn get_age(&self, date: Date) -> u32 {
        if let Some(death_date) = self.death_date {
            death_date
        } else {
            date
        }
        .get_years_since(self.birth_date)
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

    pub fn set_death_date(&mut self, death_date: Date) {
        if self.is_dead() {
            panic!("Character is already dead!");
        }

        self.death_date = Some(death_date);
    }

    pub fn get_building_relations_mut(&mut self) -> &mut Vec<BuildingRelation> {
        &mut self.building_relations
    }

    pub fn relocate(&mut self, building_id: BuildingId) {
        self.remove_occupancy();

        self.building_relations
            .push(BuildingRelation::new(Occupant, building_id));
    }

    pub fn remove_occupancy(&mut self) {
        self.building_relations
            .retain(|relation| *relation.relation_type() != Occupant);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn age_before_death() {
        let mut character = Character::new(CharacterId::new(0));
        character.set_birth_date(Date::new(10));

        assert_eq!(character.get_age(Date::new(52)), 42);
    }

    #[test]
    fn age_after_death() {
        let mut character = Character::new(CharacterId::new(0));
        character.set_birth_date(Date::new(10));
        character.set_death_date(Date::new(52));

        assert_eq!(character.get_age(Date::new(100)), 42);
    }
}
