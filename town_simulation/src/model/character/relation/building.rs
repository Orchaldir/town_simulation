use crate::model::building::BuildingId;
use derive_getters::Getters;
use derive_more::Constructor;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum BuildingRelationType {
    Builder,
    Owner,
    ExOwner,
    Occupant,
}

#[derive(Constructor, Getters, Copy, Clone, Debug, Eq, PartialEq)]
pub struct BuildingRelation {
    relation_type: BuildingRelationType,
    id: BuildingId,
}

impl PartialOrd<Self> for BuildingRelation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.relation_type.partial_cmp(&other.relation_type)
    }
}

impl Ord for BuildingRelation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.relation_type.cmp(&other.relation_type)
    }
}
