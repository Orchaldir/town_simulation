#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::response::content::Html;
use rocket::response::Redirect;
use rocket::State;
use std::sync::Mutex;
use town_simulation::generation::name::character::CharacterNameGenerator;
use town_simulation::model::character::relation::Relation;
use town_simulation::model::character::{Character, CharacterId, CharacterMgr};
use town_simulation::model::time::Date;
use town_simulation::simulation::simulate_year;
use town_simulation::usecase::character::birth::set_birth_date;
use town_simulation::usecase::character::{set_gender_based_on_id, set_generated_name};
use town_simulation::SimulationData;

struct ViewerData {
    data: Mutex<SimulationData>,
}

#[get("/")]
fn get_overview(data: &State<ViewerData>) -> Html<String> {
    let data = data.data.lock().expect("lock shared data");
    Html(format!(
        "<!DOCTYPE html>
<html>
 <head>
  <link rel=\"stylesheet\" href=\"static/style.css\">
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
    ))
}

#[get("/simulate")]
fn simulate(data: &State<ViewerData>) -> Redirect {
    let mut data = data.data.lock().expect("lock shared data");
    simulate_year(&mut data);
    Redirect::to(uri!(get_overview()))
}

#[get("/")]
fn get_characters(data: &State<ViewerData>) -> Html<String> {
    let manager = &data
        .data
        .lock()
        .expect("lock shared data")
        .character_manager;
    Html(format!(
        "<!DOCTYPE html>
<html>
 <head>
  <link rel=\"stylesheet\" href=\"static/style.css\">
 </head>
 <body>
  <h1>Characters</h1>
  <p>The town has {} characters:</p>
  <ul>
    {}
  </ul>
  <p><a href=\"/\">Back</a></p>
 </body>
</html>
",
        manager.get_all().len(),
        show_character_list(manager.get_all()),
    ))
}

fn show_character_list(characters: &[Character]) -> String {
    let vector: Vec<String> = characters
        .iter()
        .map(|c| show_character_in_list(c))
        .collect();

    vector.join("\n")
}

fn show_character_in_list(character: &Character) -> String {
    format!(
        "   <li><a href=\"/character/{}\">{}</a></li>",
        character.id().id(),
        show_character_name(character),
    )
}

fn show_character_name(character: &Character) -> String {
    if character.is_alive() {
        character.name().to_string()
    } else {
        format!("<del>{}</del>", character.name())
    }
}

#[get("/<id>")]
fn get_character(id: usize, data: &State<ViewerData>) -> Html<String> {
    let manager = &data
        .data
        .lock()
        .expect("lock shared data")
        .character_manager;

    if let Some(character) = manager.get(CharacterId::new(id)) {
        Html(format!(
            "<!DOCTYPE html>
<html>
 <head>
  <link rel=\"stylesheet\" href=\"static/style.css\">
 </head>
 <body>
  <h1>{}</h1>
  <h2>General</h2>
  <p><b>Id:</b> {}</p>
  <p><b>Gender:</b> {:?}</p>
  <p><b>Birth Date:</b> {}</p>{}
  <h2>Relations</h2>
  <ul>
    {}
  </ul>
  <a href=\"/character\">Back</a>
 </body>
</html>
",
            character.name(),
            character.id().id(),
            character.gender(),
            character.birth_date().get_year(),
            show_death(character),
            show_relations(manager, character),
        ))
    } else {
        Html(format!(
            "<!DOCTYPE html>
<html>
 <head>
  <link rel=\"stylesheet\" href=\"static/style.css\">
 </head>
 <body>
  <h1>Unknown Character {}!</h1>
  <a href=\"/\">Back</a>
 </body>
</html>
",
            id,
        ))
    }
}

fn show_death(character: &Character) -> String {
    if let Some(date) = character.death_date() {
        format!("\n<p><b>Death Date:</b> {}</p>", date.get_year())
    } else {
        "".to_string()
    }
}

fn show_relations(manager: &CharacterMgr, character: &Character) -> String {
    let vector: Vec<String> = character
        .relations
        .iter()
        .map(|r| show_relation(manager, r))
        .collect();

    vector.join("\n")
}

fn show_relation(manager: &CharacterMgr, relation: &Relation) -> String {
    let other = manager.get(*relation.id()).unwrap();
    format!(
        "   <li><a href=\"/character/{}\">{}</a> ({})</li>",
        relation.id().id(),
        show_character_name(other),
        relation
            .relation_type()
            .get_gender_specific_string(*other.gender()),
    )
}

#[rocket::main]
async fn main() {
    let simulation_data = init_simulation(Date::new(1800), 100, 20);

    let data = ViewerData {
        data: Mutex::new(simulation_data),
    };

    if let Err(e) = rocket::build()
        .manage(data)
        .mount("/static", FileServer::from("town_viewer/static/"))
        .mount("/", routes![get_overview, simulate])
        .mount("/character", routes![get_characters, get_character])
        .launch()
        .await
    {
        println!("Rocket didn't launch!");
        drop(e);
    };
}

fn init_simulation(start_date: Date, years: u32, characters: u32) -> SimulationData {
    let character_name_generator = CharacterNameGenerator::load("resources/names/english");
    let character_manager = init_characters(&character_name_generator, start_date, characters);

    let mut simulation_data = SimulationData {
        character_manager,
        character_name_generator,
        date: start_date,
    };

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
