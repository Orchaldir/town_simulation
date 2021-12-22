use crate::model::character::gender::Gender;
use crate::model::character::CharacterId;
use derive_getters::Getters;
use derive_more::Constructor;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RelationType {
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

impl RelationType {
    pub fn reverse(&self) -> RelationType {
        match self {
            RelationType::GrandParent => RelationType::GrandChild,
            RelationType::Pibling => RelationType::Nibling,
            RelationType::Nibling => RelationType::Pibling,
            RelationType::Parent => RelationType::Child,
            RelationType::Cousin => RelationType::Cousin,
            RelationType::Sibling => RelationType::Sibling,
            RelationType::Child => RelationType::Parent,
            RelationType::GrandChild => RelationType::GrandParent,
        }
    }

    pub fn get_gender_specific_string(&self, gender: Gender) -> &str {
        if gender == Gender::Male {
            match self {
                RelationType::GrandParent => "grandfather",
                RelationType::Pibling => "uncle",
                RelationType::Nibling => "nephew",
                RelationType::Parent => "father",
                RelationType::Cousin => "cousin",
                RelationType::Sibling => "brother",
                RelationType::Child => "son",
                RelationType::GrandChild => "grandson",
            }
        } else {
            match self {
                RelationType::GrandParent => "grandmother",
                RelationType::Pibling => "aunt",
                RelationType::Nibling => "niece",
                RelationType::Parent => "mother",
                RelationType::Cousin => "cousin",
                RelationType::Sibling => "sister",
                RelationType::Child => "daughter",
                RelationType::GrandChild => "granddaughter",
            }
        }
    }
}

#[derive(Constructor, Getters, Copy, Clone, Debug, PartialEq)]
pub struct Relation {
    relation_type: RelationType,
    id: CharacterId,
}
