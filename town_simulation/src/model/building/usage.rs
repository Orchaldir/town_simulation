use crate::model::character::CharacterId;
use derive_getters::Getters;
use derive_more::Constructor;

#[derive(Constructor, Getters, Clone, Debug, PartialEq)]
pub struct Home {
    occupants: Vec<CharacterId>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuildingUsage {
    Apartments(Vec<Home>),
    House(Home),
}
