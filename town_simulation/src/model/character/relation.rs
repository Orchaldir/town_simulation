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
}

#[derive(Constructor, Getters, Copy, Clone, Debug, PartialEq)]
pub struct Relation {
    relation_type: RelationType,
    id: CharacterId,
}
