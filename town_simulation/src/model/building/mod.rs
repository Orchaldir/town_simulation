use crate::model::character::CharacterId;
use crate::model::time::Date;
use derive_getters::Getters;
use derive_more::Constructor;

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
    construction_date: Date,
    builder: CharacterId,
    owner: CharacterId,
}

impl Building {
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
        construction_date: Date,
        builder: CharacterId,
        owner: CharacterId,
    ) -> BuildingId {
        let id = BuildingId::new(self.buildings.len());
        let building = Building::new(id, construction_date, builder, owner);
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