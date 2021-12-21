#[macro_use]
extern crate rocket;

use rocket::response::content::Html;
use rocket::State;
use std::sync::Mutex;
use town_simulation::model::character::CharacterMgr;
use town_simulation::usecase::character::create_child;

struct ViewerData {
    characters: Mutex<CharacterMgr>,
}

#[get("/")]
fn get_characters(data: &State<ViewerData>) -> Html<String> {
    let lock = data.characters.lock().expect("lock shared data");
    Html(format!("<h1>Characters</h1><p>The town has {} characters!</p><p>Click <a href=\"/add\">here</a> to add another!</p>", lock.get_all().len()))
}

#[get("/add")]
fn add_character(data: &State<ViewerData>) -> String {
    let mut lock = data.characters.lock().expect("lock shared data");
    let id = lock.create();
    format!("Add character with id {}!", id.id())
}

#[rocket::main]
async fn main() {
    let characters = init_characters();
    let data = ViewerData {
        characters: Mutex::new(characters),
    };

    if let Err(e) = rocket::build()
        .manage(data)
        .mount("/", routes![get_characters, add_character])
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
