use crate::model::character::gender::Gender;
use crate::model::character::CharacterId;
use derive_getters::Getters;
use derive_more::Constructor;
use RelativeType::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RelativeType {
    GrandParent,
    /// Uncle or Aunt
    Pibling,
    /// Nephew or Niece
    Nibling,
    Parent,
    Cousin,
    Sibling,
    Child,
    GrandChild,
}

impl RelativeType {
    pub fn reverse(&self) -> Self {
        match self {
            GrandParent => GrandChild,
            Pibling => Nibling,
            Nibling => Pibling,
            Parent => Child,
            Cousin => Cousin,
            Sibling => Sibling,
            Child => Parent,
            GrandChild => GrandParent,
        }
    }

    pub fn get_gender_specific_string(&self, gender: Gender) -> &str {
        if gender == Gender::Male {
            match self {
                GrandParent => "grandfather",
                Pibling => "uncle",
                Nibling => "nephew",
                Parent => "father",
                Cousin => "cousin",
                Sibling => "brother",
                Child => "son",
                GrandChild => "grandson",
            }
        } else {
            match self {
                GrandParent => "grandmother",
                Pibling => "aunt",
                Nibling => "niece",
                Parent => "mother",
                Cousin => "cousin",
                Sibling => "sister",
                Child => "daughter",
                GrandChild => "granddaughter",
            }
        }
    }
}

#[derive(Constructor, Getters, Copy, Clone, Debug, PartialEq)]
pub struct Relation {
    relation_type: RelativeType,
    id: CharacterId,
}
