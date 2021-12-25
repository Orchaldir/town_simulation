use town_simulation::generation::name::character::CharacterNameGenerator;
use town_simulation::model::building::usage::{BuildingUsage, Home};
use town_simulation::model::building::BuildingMgr;
use town_simulation::model::character::{CharacterId, CharacterMgr};
use town_simulation::model::time::Date;
use town_simulation::simulation::simulate_year;
use town_simulation::usecase::character::birth::set_birth_date;
use town_simulation::usecase::character::{set_gender_based_on_id, set_generated_name};
use town_simulation::SimulationData;

pub fn init_simulation(mut start_date: Date, years: u32, characters: u32) -> SimulationData {
    let building_manager = init_buildings(start_date);
    let character_name_generator = CharacterNameGenerator::load("resources/names/english");
    let character_manager = init_characters(&character_name_generator, start_date, characters);

    start_date.increase_by(20);

    let mut simulation_data = SimulationData {
        building_manager,
        character_manager,
        character_name_generator,
        date: start_date,
    };

    for _i in 0..years {
        simulate_year(&mut simulation_data);
    }

    simulation_data
}

fn init_buildings(date: Date) -> BuildingMgr {
    let mut manager = BuildingMgr::default();
    let id0 = CharacterId::new(0);
    let id1 = CharacterId::new(1);
    let id2 = CharacterId::new(2);
    let id3 = CharacterId::new(3);
    let usage0 = BuildingUsage::House(Home::new(vec![id0, id1]));
    let usage1 = BuildingUsage::Apartments(vec![Home::new(vec![id2]), Home::new(vec![id3])]);

    manager.create(usage0, date, id0, id0);
    manager.create(usage1, date, id0, id2);

    manager
}

fn init_characters(names: &CharacterNameGenerator, date: Date, characters: u32) -> CharacterMgr {
    let mut manager = CharacterMgr::default();

    for _i in 0..characters {
        init_character(&mut manager, names, date);
    }

    manager
}

fn init_character(
    manager: &mut CharacterMgr,
    name_generator: &CharacterNameGenerator,
    date: Date,
) -> CharacterId {
    let id = manager.create();
    set_birth_date(manager, id, date);
    set_gender_based_on_id(manager, id);
    set_generated_name(manager, name_generator, id);
    id
}
