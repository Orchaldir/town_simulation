use crate::model::building::usage::BuildingUsage;
use crate::model::character::CharacterId;
use crate::model::time::Date;
use derive_getters::Getters;
use derive_more::Constructor;

pub mod usage;

#[derive(Constructor, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct BuildingId(usize);

impl BuildingId {
    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Constructor, Getters, Clone, Debug, PartialEq)]
pub struct Building {
    id: BuildingId,
    usage: BuildingUsage,
    construction_date: Date,
    builder: CharacterId,
    owner: CharacterId,
}

impl Building {
    pub fn get_age(&self, date: Date) -> u32 {
        date.get_years_since(self.construction_date)
    }

    pub fn get_usage_mut(&mut self) -> &mut BuildingUsage {
        &mut self.usage
    }

    pub fn remove_occupant(&mut self, id: CharacterId) {
        match &mut self.usage {
            BuildingUsage::Apartments(homes) => {
                homes.iter_mut().for_each(|home| home.remove_occupant(id))
            }
            BuildingUsage::House(home) => home.remove_occupant(id),
        }
    }

    pub fn update_owner(&mut self, owner: CharacterId) {
        self.owner = owner;
    }
}

#[derive(Default, Debug)]
pub struct BuildingMgr {
    buildings: Vec<Building>,
}

impl BuildingMgr {
    pub fn create(
        &mut self,
        usage: BuildingUsage,
        construction_date: Date,
        builder: CharacterId,
        owner: CharacterId,
    ) -> BuildingId {
        let id = BuildingId::new(self.buildings.len());
        let building = Building::new(id, usage, construction_date, builder, owner);
        self.buildings.push(building);
        id
    }

    pub fn get_all(&self) -> &Vec<Building> {
        &self.buildings
    }

    pub fn get(&self, id: BuildingId) -> Option<&Building> {
        self.buildings.get(id.0)
    }

    pub fn get_mut(&mut self, id: BuildingId) -> Option<&mut Building> {
        self.buildings.get_mut(id.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_age() {
        let id = BuildingId::new(0);
        let character_id = CharacterId::new(0);
        let building = Building::new(
            id,
            BuildingUsage::house(),
            Date::new(10),
            character_id,
            character_id,
        );

        assert_eq!(building.get_age(Date::new(52)), 42);
    }
}
