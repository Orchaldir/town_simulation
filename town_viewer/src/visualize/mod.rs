use town_simulation::SimulationData;

pub fn visualize_overview(data: &SimulationData) -> String {
    format!(
        "<!DOCTYPE html>
<html>
 <head>
  <link rel=\"stylesheet\" href=\"/static/style.css\">
 </head>
 <body>
  <h1>Town Simulation</h1>
  <h2>Overview</h2>
  <p><b>Year:</b> {}</p>
  <p><b>Characters</b>: <a href=\"/character\">{}</a></p>
  <h2>Actions</h2>
  <p><a href=\"/simulate\">Simulate</a></p>
 </body>
</html>
",
        data.date.get_year(),
        data.character_manager.get_all().len()
    )
}
