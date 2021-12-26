use town_simulation::SimulationData;

pub mod building;
pub mod character;
pub mod town;

pub fn html(body: String) -> String {
    format!(
        "<!DOCTYPE html>
<html>
 <head>
  <link rel=\"stylesheet\" href=\"/static/style.css\">
 </head>
 <body>
  {}
 </body>
</html>
",
        body
    )
}

pub fn visualize_overview(data: &SimulationData) -> String {
    html(format!(
        "
  <h1>Town Simulation</h1>
  <h2>Overview</h2>
  <p><b>Year:</b> {}</p>
  <p><b>Buildings</b>: <a href=\"/building\">{}</a></p>
  <p><b>Characters</b>: <a href=\"/character\">{}</a></p>
  <p><<a href=\"/town\">Town Map</a></p>
  <h2>Actions</h2>
  <p><a href=\"/simulate\">Simulate</a></p>",
        data.date.get_year(),
        data.building_manager.get_all().len(),
        data.character_manager.get_all().len(),
    ))
}
