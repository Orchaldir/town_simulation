use crate::model::character::relation::family::RelationType;
use crate::model::character::CharacterId;
use derive_getters::Getters;
use derive_more::Constructor;

pub mod family;

#[derive(Constructor, Getters, Copy, Clone, Debug, PartialEq)]
pub struct Relation {
    relation_type: RelationType,
    id: CharacterId,
}
