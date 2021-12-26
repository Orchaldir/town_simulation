use crate::visualize::html;
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
   <td>
    {:?}
   </td>",
        block,
    )
}
