use crate::visualize::html;
use town_simulation::model::building::Building;
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
