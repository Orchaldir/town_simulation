use crate::model::character::gender::Gender;
use crate::model::character::gender::Gender::Male;
use crate::model::character::relation::family::RelativeType;
use crate::model::character::CharacterId;
use derive_getters::Getters;
use derive_more::Constructor;
use std::cmp::Ordering;
use RelationType::*;

pub mod family;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum RelationType {
    InLaw(RelativeType),
    Relative(RelativeType),
    Spouse,
}

impl RelationType {
    pub fn reverse(&self) -> Self {
        match self {
            InLaw(relative_type) => InLaw(relative_type.reverse()),
            Relative(relative_type) => Relative(relative_type.reverse()),
            Spouse => Spouse,
        }
    }

    pub fn is_in_law(&self) -> bool {
        matches!(self, InLaw(..))
    }

    pub fn is_relative(&self) -> bool {
        matches!(self, Relative(..))
    }

    pub fn get_gender_specific_string(&self, gender: Gender) -> String {
        match self {
            InLaw(relative_type) => format!(
                "{}-in-law",
                relative_type.get_gender_specific_string(gender)
            ),
            Relative(relative_type) => relative_type.get_gender_specific_string(gender).to_string(),
            Spouse => if gender == Male { "husband" } else { "wife" }.to_string(),
        }
    }
}

#[derive(Constructor, Getters, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Relation {
    relation_type: RelationType,
    id: CharacterId,
}

impl Relation {
    pub fn to_in_law(&self) -> Option<Self> {
        match self.relation_type {
            Relative(relative_type) => Some(Self::new(InLaw(relative_type), self.id)),
            _ => None,
        }
    }
}

impl PartialOrd<Self> for Relation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.relation_type.partial_cmp(&other.relation_type)
    }
}

impl Ord for Relation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.relation_type.cmp(&other.relation_type)
    }
}
