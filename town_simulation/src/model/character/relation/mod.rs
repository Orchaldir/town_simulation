use crate::model::character::gender::Gender;
use crate::model::character::gender::Gender::Male;
use crate::model::character::relation::family::RelativeType;
use crate::model::character::CharacterId;
use derive_getters::Getters;
use derive_more::Constructor;
use RelationType::*;

pub mod family;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RelationType {
    Relative(RelativeType),
    Spouse,
}

impl RelationType {
    pub fn reverse(&self) -> Self {
        match self {
            Relative(relative_type) => Relative(relative_type.reverse()),
            Spouse => Spouse,
        }
    }

    pub fn get_gender_specific_string(&self, gender: Gender) -> &str {
        match self {
            Relative(relative_type) => relative_type.get_gender_specific_string(gender),
            Spouse => {
                if gender == Male {
                    "husband"
                } else {
                    "wife"
                }
            }
        }
    }
}

#[derive(Constructor, Getters, Copy, Clone, Debug, PartialEq)]
pub struct Relation {
    relation_type: RelationType,
    id: CharacterId,
}
