#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::response::content::Html;
use rocket::State;
use std::sync::Mutex;
use town_simulation::model::character::{Character, CharacterId, CharacterMgr};
use town_simulation::usecase::character::create_child;

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
  <a href=\"/\">Back</a>
 </body>
</html>
",
            character.id().id(),
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
    let grandfather0 = manager.create();
    let grandmother0 = manager.create();
    let grandfather1 = manager.create();
    let grandmother1 = manager.create();

    // generation 1
    let father = create_child(&mut manager, grandfather0, grandmother0);
    let aunt = create_child(&mut manager, grandfather0, grandmother0);
    let mother = create_child(&mut manager, grandfather1, grandmother1);
    create_child(&mut manager, grandfather1, grandmother1);
    let husband_aunt = manager.create();

    // generation 2
    create_child(&mut manager, father, mother);
    create_child(&mut manager, father, mother);
    create_child(&mut manager, father, mother);
    create_child(&mut manager, husband_aunt, aunt);

    manager
}
