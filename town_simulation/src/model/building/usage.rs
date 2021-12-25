use crate::model::character::CharacterId;
use derive_getters::Getters;
use derive_more::Constructor;

#[derive(Constructor, Default, Getters, Clone, Debug, PartialEq)]
pub struct Home {
    occupants: Vec<CharacterId>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuildingUsage {
    Apartments(Vec<Home>),
    House(Home),
}

impl BuildingUsage {
    pub fn house() -> Self {
        BuildingUsage::House(Home::default())
    }
}
