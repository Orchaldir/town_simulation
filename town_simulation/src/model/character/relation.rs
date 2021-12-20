use crate::model::character::CharacterId;
use derive_more::Constructor;

#[derive(Clone, Debug, PartialEq)]
pub enum RelationType {
    GrandParent,
    /// Uncle or Aunt
    Pibling,
    Cousin,
    Sibling,
    Child,
    GrandChild,
}

#[derive(Constructor, Clone, Debug, PartialEq)]
pub struct Relation {
    relation_type: RelationType,
    id: CharacterId,
}
