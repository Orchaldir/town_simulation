#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::response::content::Html;
use rocket::State;
use std::sync::Mutex;
use town_simulation::generation::name::character::CharacterNameGenerator;
use town_simulation::model::character::gender::Gender;
use town_simulation::model::character::gender::Gender::{Female, Male};
use town_simulation::model::character::relation::Relation;
use town_simulation::model::character::{Character, CharacterId, CharacterMgr};
use town_simulation::usecase::character::marriage::marry;
use town_simulation::usecase::character::{create_child, set_gender, set_generated_name};

struct ViewerData {
    characters: Mutex<CharacterMgr>,
}

#[get("/")]
fn get_characters(data: &State<ViewerData>) -> Html<String> {
    let lock = data.characters.lock().expect("lock shared data");
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
 </body>
</html>
",
        lock.get_all().len(),
        get_character_list(lock.get_all()),
    ))
}

fn get_character_list(characters: &[Character]) -> String {
    let vector: Vec<String> = characters
        .iter()
        .map(|c| get_character_in_list(c))
        .collect();

    vector.join("\n")
}

fn get_character_in_list(character: &Character) -> String {
    format!(
        "   <li><a href=\"/{}\">{}</a></li>",
        character.id().id(),
        character.name(),
    )
}

#[get("/<id>")]
fn get_character(id: usize, data: &State<ViewerData>) -> Html<String> {
    let lock = data.characters.lock().expect("lock shared data");

    if let Some(character) = lock.get(CharacterId::new(id)) {
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
  <h2>Relations</h2>
  <ul>
    {}
  </ul>
  <a href=\"/\">Back</a>
 </body>
</html>
",
            character.name(),
            character.id().id(),
            character.gender(),
            show_relations(&lock, character),
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
        "   <li><a href=\"/{}\">{}</a> ({})</li>",
        relation.id().id(),
        other.name(),
        relation
            .relation_type()
            .get_gender_specific_string(*other.gender()),
    )
}

#[rocket::main]
async fn main() {
    let name_generator = CharacterNameGenerator::load("resources/names/english");
    let characters = init_characters(&name_generator);
    let data = ViewerData {
        characters: Mutex::new(characters),
    };

    if let Err(e) = rocket::build()
        .manage(data)
        .mount("/static", FileServer::from("town_viewer/static/"))
        .mount("/", routes![get_characters, get_character])
        .launch()
        .await
    {
        println!("Rocket didn't launch!");
        drop(e);
    };
}

fn init_characters(names: &CharacterNameGenerator) -> CharacterMgr {
    let mut manager = CharacterMgr::default();

    // generation 0
    let grandfather0 = init_character(&mut manager, names, Male);
    let grandmother0 = init_character(&mut manager, names, Female);
    let grandfather1 = init_character(&mut manager, names, Male);
    let grandmother1 = init_character(&mut manager, names, Female);

    marry(&mut manager, grandfather0, grandmother0);
    marry(&mut manager, grandfather1, grandmother1);

    // generation 1
    let father = init_son(&mut manager, names, grandfather0, grandmother0);
    let aunt = init_daughter(&mut manager, names, grandfather0, grandmother0);
    let mother = init_daughter(&mut manager, names, grandfather1, grandmother1);
    init_son(&mut manager, names, grandfather1, grandmother1);
    let husband_aunt = init_character(&mut manager, names, Male);

    marry(&mut manager, father, mother);
    marry(&mut manager, husband_aunt, aunt);

    // generation 2
    init_child(&mut manager, names, father, mother, Male);
    init_child(&mut manager, names, father, mother, Female);
    init_child(&mut manager, names, father, mother, Male);
    init_child(&mut manager, names, husband_aunt, aunt, Female);

    manager
}

fn init_character(
    manager: &mut CharacterMgr,
    name_generator: &CharacterNameGenerator,
    gender: Gender,
) -> CharacterId {
    let id = manager.create();
    set_gender(manager, id, gender);
    set_generated_name(manager, name_generator, id);
    id
}

fn init_son(
    manager: &mut CharacterMgr,
    name_generator: &CharacterNameGenerator,
    father: CharacterId,
    mother: CharacterId,
) -> CharacterId {
    init_child(manager, name_generator, father, mother, Male)
}

fn init_daughter(
    manager: &mut CharacterMgr,
    name_generator: &CharacterNameGenerator,
    father: CharacterId,
    mother: CharacterId,
) -> CharacterId {
    init_child(manager, name_generator, father, mother, Female)
}

fn init_child(
    manager: &mut CharacterMgr,
    name_generator: &CharacterNameGenerator,
    father: CharacterId,
    mother: CharacterId,
    gender: Gender,
) -> CharacterId {
    let id = create_child(manager, father, mother);
    set_gender(manager, id, gender);
    set_generated_name(manager, name_generator, id);
    id
}
