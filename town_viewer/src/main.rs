#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::response::content::Html;
use rocket::State;
use std::sync::Mutex;
use town_simulation::model::character::gender::Gender;
use town_simulation::model::character::gender::Gender::{Female, Male};
use town_simulation::model::character::relation::Relation;
use town_simulation::model::character::{Character, CharacterId, CharacterMgr};
use town_simulation::usecase::character::{create_child, set_gender};

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
        "   <li><a href=\"/{0}\">Character {0}</a></li>",
        character.id().id()
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
  <h1>Character {}</h1>
  <h2>Relations</h2>
  <ul>
    {}
  </ul>
  <a href=\"/\">Back</a>
 </body>
</html>
",
            character.id().id(),
            show_relations(character),
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

fn show_relations(character: &Character) -> String {
    let vector: Vec<String> = character
        .relations
        .iter()
        .map(|r| show_relation(r))
        .collect();

    vector.join("\n")
}

fn show_relation(relation: &Relation) -> String {
    format!(
        "   <li>{0:?}: <a href=\"/{1}\">Character {1}</a></li>",
        relation.relation_type(),
        relation.id().id(),
    )
}

#[rocket::main]
async fn main() {
    let characters = init_characters();
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

fn init_characters() -> CharacterMgr {
    let mut manager = CharacterMgr::default();

    // generation 0
    let grandfather0 = init_character(&mut manager, Male);
    let grandmother0 = init_character(&mut manager, Female);
    let grandfather1 = init_character(&mut manager, Male);
    let grandmother1 = init_character(&mut manager, Female);

    // generation 1
    let father = init_son(&mut manager, grandfather0, grandmother0);
    let aunt = init_daughter(&mut manager, grandfather0, grandmother0);
    let mother = init_daughter(&mut manager, grandfather1, grandmother1);
    init_son(&mut manager, grandfather1, grandmother1);
    let husband_aunt = init_character(&mut manager, Female);

    // generation 2
    init_child(&mut manager, father, mother, Male);
    init_child(&mut manager, father, mother, Female);
    init_child(&mut manager, father, mother, Male);
    init_child(&mut manager, husband_aunt, aunt, Female);

    manager
}

fn init_character(manager: &mut CharacterMgr, gender: Gender) -> CharacterId {
    let id = manager.create();
    set_gender(manager, id, gender);
    id
}

fn init_son(manager: &mut CharacterMgr, father: CharacterId, mother: CharacterId) -> CharacterId {
    init_child(manager, father, mother, Male)
}

fn init_daughter(
    manager: &mut CharacterMgr,
    father: CharacterId,
    mother: CharacterId,
) -> CharacterId {
    init_child(manager, father, mother, Female)
}

fn init_child(
    manager: &mut CharacterMgr,
    father: CharacterId,
    mother: CharacterId,
    gender: Gender,
) -> CharacterId {
    let id = create_child(manager, father, mother);
    set_gender(manager, id, gender);
    id
}
