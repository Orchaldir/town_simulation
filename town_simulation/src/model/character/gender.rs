use crate::model::character::gender::Gender::{Female, Male};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn reverse(&self) -> Gender {
        match self {
            Male => Female,
            Female => Male,
        }
    }

    pub fn is_reverse(&self, gender: Gender) -> bool {
        self.reverse() == gender
    }
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}
