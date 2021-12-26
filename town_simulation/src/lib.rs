extern crate derive_more;

use crate::generation::name::character::CharacterNameGenerator;
use crate::model::building::BuildingMgr;
use crate::model::character::CharacterMgr;
use crate::model::time::Date;
use crate::model::town::map::TownMap;

pub mod generation;
pub mod model;
pub mod simulation;
pub mod usecase;

pub struct SimulationData {
    pub building_manager: BuildingMgr,
    pub character_manager: CharacterMgr,
    pub character_name_generator: CharacterNameGenerator,
    pub date: Date,
    pub map: TownMap,
}
