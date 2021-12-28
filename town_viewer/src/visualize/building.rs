use crate::visualize::character::show_character_id_link;
use crate::visualize::html;
use town_simulation::model::building::usage::{BuildingUsage, Home};
use town_simulation::model::building::{Building, BuildingId, BuildingMgr};
use town_simulation::model::character::{CharacterId, CharacterMgr};
use town_simulation::model::time::Date;
use town_simulation::SimulationData;

pub fn visualize_buildings(data: &SimulationData) -> String {
    let manager = &data.building_manager;

    html(format!(
        "
  <h1>Buildings</h1>
  <p><b>Total:</b> {}</p>
  <ul>
    {}
  </ul>
  <p><a href=\"/\">Back</a></p>",
        manager.get_all().len(),
        show_building_list(manager.get_all(), data.date),
    ))
}

pub fn visualize_building(data: &SimulationData, id: usize) -> String {
    let manager = &data.building_manager;
    let building_id = BuildingId::new(id);

    if let Some(building) = manager.get(building_id) {
        html(format!(
            "
  <h1>Building {0}</h1>
  <h2>General</h2>
  <p><b>Id:</b> {0}</p>
  <p><b>Construction Date:</b> {1}</p>
  <p><b>Age:</b> {2}</p>
  <p><b>Builder:</b> {3}</p>
  <p><b>Owner:</b> {4}</p>{5}
  {6}
  <a href=\"/building\">Back</a>",
            id,
            building.construction_date().get_year(),
            building.get_age(data.date),
            show_character_id_link(&data.character_manager, *building.builder()),
            show_character_id_link(&data.character_manager, *building.owner()),
            show_previous_owners(&data.character_manager, building.previous_owners()),
            show_usage(&data.character_manager, building.usage()),
        ))
    } else {
        html(format!(
            "
  <h1>Unknown Building {}!</h1>
  <a href=\"/building\">Back</a>",
            id,
        ))
    }
}

fn show_building_list(building: &[Building], date: Date) -> String {
    let vector: Vec<String> = building
        .iter()
        .map(|b| show_building_in_list(b, date))
        .collect();

    vector.join("\n")
}

fn show_building_in_list(building: &Building, date: Date) -> String {
    format!(
        "   <li>{} (Age: {})</li>",
        show_building_link(building),
        building.get_age(date),
    )
}

pub fn show_building_id_link(manager: &BuildingMgr, id: BuildingId) -> String {
    show_building_link(manager.get(id).unwrap())
}

pub fn show_building_link(building: &Building) -> String {
    format!(
        "<a href=\"/building/{0}\">{1} {0}</a>",
        building.id().id(),
        building.usage()
    )
}

fn show_previous_owners(manager: &CharacterMgr, previous_owners: &[CharacterId]) -> String {
    if previous_owners.is_empty() {
        "".to_string()
    } else {
        let vector: Vec<String> = previous_owners
            .iter()
            .map(|id| show_occupant(manager, *id))
            .collect();

        format!(
            "\n<p><b>Former Owners:</b></p>\n<ul>{}</ul>",
            vector.join("\n")
        )
    }
}

fn show_usage(manager: &CharacterMgr, usage: &BuildingUsage) -> String {
    format!(
        "<p><b>Usage:</b> {}</p><ul>{}</ul>",
        usage,
        match usage {
            BuildingUsage::Apartments(homes) => show_homes(manager, homes),
            BuildingUsage::House(home) => show_occupants(manager, home),
        },
    )
}

fn show_homes(manager: &CharacterMgr, homes: &[Home]) -> String {
    let vector: Vec<String> = homes.iter().map(|home| show_home(manager, home)).collect();

    vector.join("\n")
}

fn show_home(manager: &CharacterMgr, home: &Home) -> String {
    format!("<li>Home<ul>{}</ul></li>", show_occupants(manager, home))
}

fn show_occupants(manager: &CharacterMgr, home: &Home) -> String {
    let vector: Vec<String> = home
        .occupants()
        .iter()
        .map(|id| show_occupant(manager, *id))
        .collect();

    vector.join("\n")
}

fn show_occupant(manager: &CharacterMgr, id: CharacterId) -> String {
    format!("<li>{}</li>", show_character_id_link(manager, id))
}
