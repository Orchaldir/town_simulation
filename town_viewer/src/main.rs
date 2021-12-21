#[macro_use]
extern crate rocket;

use rocket::State;
use town_simulation::model::character::CharacterMgr;
use town_simulation::usecase::character::create_child;

#[get("/")]
fn get_characters(characters: &State<CharacterMgr>) -> String {
    format!("The town has {} characters!", characters.get_all().len())
}

#[rocket::main]
async fn main() {
    let characters = init_characters();

    if let Err(e) = rocket::build()
        .manage(characters)
        .mount("/", routes![get_characters])
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
