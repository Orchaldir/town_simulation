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

#[derive(Constructor, Getters, Copy, Clone, Debug, PartialEq)]
pub struct Relation {
    relation_type: RelationType,
    id: CharacterId,
}
