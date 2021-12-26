use crate::visualize::html;
use town_simulation::model::building::usage::BuildingUsage;
use town_simulation::model::building::BuildingId;
use town_simulation::model::town::map::{TownBlock, TownLot};
use town_simulation::SimulationData;

pub fn visualize_town(data: &SimulationData) -> String {
    html(format!(
        "
  <h1>Town</h1>
  <table>
   {}
  </table>
  <p><a href=\"/\">Back</a></p>",
        visualize_map(data),
    ))
}

pub fn visualize_map(data: &SimulationData) -> String {
    let mut rows = Vec::new();

    for row in 0..*data.map.height() {
        rows.push(visualize_row(data, row));
    }

    rows.join("\n")
}

pub fn visualize_row(data: &SimulationData, row: usize) -> String {
    let mut columns = Vec::new();

    for column in 0..*data.map.width() {
        columns.push(visualize_block(data, row, column));
    }

    format!(
        "
   <tr>
    {}
   </tr>",
        columns.join("\n"),
    )
}

pub fn visualize_block(data: &SimulationData, row: usize, column: usize) -> String {
    let block = data.map.get_block(row, column);

    format!(
        "
   <td class=\"block\">
    {}
   </td>",
        match block {
            TownBlock::EmptyBlock => "E".to_string(),
            TownBlock::SmallBuildings(buildings) => visualize_town_lots(data, buildings),
        },
    )
}

pub fn visualize_town_lots(data: &SimulationData, town_lots: &[TownLot; 4]) -> String {
    format!(
        "
<table class=\"lots\">
 <tr>
  <td class=\"lot\">{}</td>
  <td class=\"lot\">{}</td>
 </tr>
 <tr>
  <td class=\"lot\">{}</td>
  <td class=\"lot\">{}</td>
 </tr>
</table>",
        visualize_town_lot(data, &town_lots[0]),
        visualize_town_lot(data, &town_lots[1]),
        visualize_town_lot(data, &town_lots[2]),
        visualize_town_lot(data, &town_lots[3]),
    )
}

pub fn visualize_town_lot(data: &SimulationData, town_lot: &TownLot) -> String {
    match town_lot {
        TownLot::EmptyLot => "E".to_string(),
        TownLot::BuildingLot(id) => visualize_building(data, *id),
    }
}

pub fn visualize_building(data: &SimulationData, id: BuildingId) -> String {
    let building = data.building_manager.get(id).unwrap();

    format!(
        "<a href=\"/building/{}\">{}</a>",
        building.id().id(),
        match building.usage() {
            BuildingUsage::Apartments(_) => visualize_icon("apartment"),
            BuildingUsage::House(_) => visualize_icon("house"),
        }
    )
}

pub fn visualize_icon(icon: &str) -> String {
    format!("<img src=\"static/icons/{0}.svg\" alt=\"{0}\">", icon)
}
