use crate::model::character::CharacterId;
use derive_getters::Getters;
use derive_more::Constructor;
use std::fmt::{Display, Formatter};
use BuildingUsage::{Apartments, House};

#[derive(Constructor, Default, Getters, Clone, Debug, PartialEq)]
pub struct Home {
    occupants: Vec<CharacterId>,
}

impl Home {
    pub fn is_empty(&self) -> bool {
        self.occupants.is_empty()
    }

    pub fn get_occupants_mut(&mut self) -> &mut Vec<CharacterId> {
        &mut self.occupants
    }

    pub fn remove_occupant(&mut self, id: CharacterId) {
        self.occupants.retain(|occupant_id| *occupant_id != id);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuildingUsage {
    Apartments(Vec<Home>),
    House(Home),
}

impl BuildingUsage {
    pub fn house() -> Self {
        House(Home::default())
    }

    pub fn is_apartments(&self) -> bool {
        matches!(self, Apartments(..))
    }

    pub fn is_house(&self) -> bool {
        matches!(self, House(..))
    }
}

impl Display for BuildingUsage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Apartments(_) => write!(f, "Apartments"),
            House(_) => write!(f, "House"),
        }
    }
}
