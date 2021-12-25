use crate::visualize::character::show_character_id_link;
use crate::visualize::html;
use town_simulation::model::building::{Building, BuildingId};
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
  <p><b>Owner:</b> {4}</p>
  <a href=\"/character\">Back</a>",
            id,
            building.construction_date().get_year(),
            building.get_age(data.date),
            show_character_id_link(&data.character_manager, *building.builder()),
            show_character_id_link(&data.character_manager, *building.owner()),
        ))
    } else {
        html(format!(
            "
  <h1>Unknown Building {}!</h1>
  <a href=\"/\">Back</a>",
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
        "   <li><a href=\"/building/{0}\">{0}</a> (Age: {1})</li>",
        building.id().id(),
        building.get_age(date),
    )
}
