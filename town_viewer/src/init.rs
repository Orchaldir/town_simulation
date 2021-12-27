use town_simulation::generation::name::character::CharacterNameGenerator;
use town_simulation::model::building::usage::{BuildingUsage, Home};
use town_simulation::model::building::BuildingMgr;
use town_simulation::model::character::{CharacterId, CharacterMgr};
use town_simulation::model::time::Date;
use town_simulation::model::town::map::TownMap;
use town_simulation::simulation::simulate_year;
use town_simulation::usecase::building::build::build;
use town_simulation::usecase::building::relocate::relocate_to_house;
use town_simulation::usecase::character::birth::set_birth_date;
use town_simulation::usecase::character::{set_gender_based_on_id, set_generated_name};
use town_simulation::SimulationData;

pub fn init_simulation(mut start_date: Date, years: u32, characters: u32) -> SimulationData {
    let character_name_generator = CharacterNameGenerator::load("resources/names/english");
    let character_manager = init_characters(&character_name_generator, start_date, characters);

    start_date.increase_by(20);

    let mut simulation_data = SimulationData {
        building_manager: BuildingMgr::default(),
        character_manager,
        character_name_generator,
        date: start_date,
        map: TownMap::empty(6, 5),
    };

    let character0 = CharacterId::new(0);
    let character1 = CharacterId::new(1);
    let character2 = CharacterId::new(2);
    let character3 = CharacterId::new(3);

    let building0 = build(
        &mut simulation_data,
        7,
        0,
        BuildingUsage::House(Home::default()),
        character0,
        character0,
    );
    let building1 = build(
        &mut simulation_data,
        7,
        1,
        BuildingUsage::House(Home::default()),
        character0,
        character2,
    );

    relocate_to_house(
        &mut simulation_data,
        vec![character0, character1],
        building0,
    );
    relocate_to_house(
        &mut simulation_data,
        vec![character2, character3],
        building1,
    );

    for _i in 0..years {
        simulate_year(&mut simulation_data);
    }

    simulation_data
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
