#[macro_use]
extern crate rocket;

use crate::init::init_simulation;
use crate::visualize::{html, visualize_overview};
use rocket::fs::FileServer;
use rocket::response::content::Html;
use rocket::response::Redirect;
use rocket::State;
use std::sync::Mutex;
use town_simulation::model::character::relation::Relation;
use town_simulation::model::character::{Character, CharacterId, CharacterMgr};
use town_simulation::model::time::Date;
use town_simulation::simulation::simulate_year;
use town_simulation::usecase::character::relation::get::{
    get_relation_to_in_laws, get_relation_to_relatives, get_spouses,
};
use town_simulation::SimulationData;

pub mod init;
pub mod visualize;

struct ViewerData {
    data: Mutex<SimulationData>,
}

#[get("/")]
fn get_overview(data: &State<ViewerData>) -> Html<String> {
    let data = data.data.lock().expect("lock shared data");
    Html(visualize_overview(&data))
}

#[get("/simulate")]
fn simulate(data: &State<ViewerData>) -> Redirect {
    let mut data = data.data.lock().expect("lock shared data");
    simulate_year(&mut data);
    Redirect::to(uri!(get_overview()))
}

#[get("/")]
fn get_characters(data: &State<ViewerData>) -> Html<String> {
    let lock = &data.data.lock().expect("lock shared data");
    let manager = &lock.character_manager;
    let total = manager.get_all().len();
    let alive = manager.get_all().iter().filter(|&c| c.is_alive()).count();
    let dead = total - alive;
    Html(html(format!(
        "
  <h1>Characters</h1>
  <p><b>Alive:</b> {}</p>
  <p><b>Dead:</b> {}</p>
  <p><b>Total:</b> {}</p>
  <ul>
    {}
  </ul>
  <p><a href=\"/\">Back</a></p>",
        alive,
        dead,
        total,
        show_character_list(manager.get_all(), lock.date),
    )))
}

fn show_character_list(characters: &[Character], date: Date) -> String {
    let vector: Vec<String> = characters
        .iter()
        .map(|c| show_character_in_list(c, date))
        .collect();

    vector.join("\n")
}

fn show_character_in_list(character: &Character, date: Date) -> String {
    format!(
        "   <li><a href=\"/character/{}\">{}</a> (Age: {})</li>",
        character.id().id(),
        show_character_name(character),
        character.get_age(date),
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
    let lock = &data.data.lock().expect("lock shared data");
    let manager = &lock.character_manager;
    let character_id = CharacterId::new(id);

    if let Some(character) = manager.get(character_id) {
        Html(html(format!(
            "
  <h1>{}</h1>
  <h2>General</h2>
  <p><b>Id:</b> {}</p>
  <p><b>Gender:</b> {:?}</p>
  <p><b>Birth Date:</b> {}</p>{}
  <p><b>Age:</b> {}</p>
  <h2>Relations</h2>{}{}{}
  <a href=\"/character\">Back</a>",
            character.name(),
            id,
            character.gender(),
            character.birth_date().get_year(),
            show_death(character),
            character.get_age(lock.date),
            show_spouse(manager, character_id),
            show_relatives(manager, character_id),
            show_in_laws(manager, character_id),
        )))
    } else {
        Html(html(format!(
            "
  <h1>Unknown Character {}!</h1>
  <a href=\"/\">Back</a>",
            id,
        )))
    }
}

fn show_death(character: &Character) -> String {
    if let Some(date) = character.death_date() {
        format!("\n<p><b>Death Date:</b> {}</p>", date.get_year())
    } else {
        "".to_string()
    }
}

fn show_spouse(manager: &CharacterMgr, character: CharacterId) -> String {
    if let Some(spouse) = get_spouses(manager, character)
        .iter()
        .map(|id| manager.get(*id))
        .flatten()
        .next()
    {
        format!(
            "\n<p><b>Spouse:</b> <a href=\"/character/{}\">{}</a></p>",
            spouse.id().id(),
            show_character_name(spouse),
        )
    } else {
        "".to_string()
    }
}

fn show_relatives(manager: &CharacterMgr, id: CharacterId) -> String {
    show_relations(manager, get_relation_to_relatives(manager, id), "Relatives")
}

fn show_in_laws(manager: &CharacterMgr, id: CharacterId) -> String {
    show_relations(manager, get_relation_to_in_laws(manager, id), "In-Laws")
}

fn show_relations(manager: &CharacterMgr, mut relations: Vec<&Relation>, text: &str) -> String {
    if relations.is_empty() {
        "".to_string()
    } else {
        relations.sort();
        let vector: Vec<String> = relations
            .iter()
            .map(|r| show_relation(manager, r))
            .collect();

        format!("\n<p><b>{}:</b></p>\n<ul>{}</ul>", text, vector.join("\n"),)
    }
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
    let simulation_data = init_simulation(Date::new(1800), 100, 50);

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
