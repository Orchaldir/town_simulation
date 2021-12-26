#[macro_use]
extern crate rocket;

use crate::init::init_simulation;
use crate::visualize::building::{visualize_building, visualize_buildings};
use crate::visualize::character::{visualize_character, visualize_characters};
use crate::visualize::town::visualize_town;
use crate::visualize::visualize_overview;
use rocket::fs::FileServer;
use rocket::response::content::Html;
use rocket::response::Redirect;
use rocket::State;
use std::sync::Mutex;
use town_simulation::model::time::Date;
use town_simulation::simulation::simulate_year;
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
fn get_buildings(data: &State<ViewerData>) -> Html<String> {
    let data = data.data.lock().expect("lock shared data");
    Html(visualize_buildings(&data))
}

#[get("/<id>")]
fn get_building(id: usize, data: &State<ViewerData>) -> Html<String> {
    let data = data.data.lock().expect("lock shared data");
    Html(visualize_building(&data, id))
}

#[get("/")]
fn get_characters(data: &State<ViewerData>) -> Html<String> {
    let data = data.data.lock().expect("lock shared data");
    Html(visualize_characters(&data))
}

#[get("/<id>")]
fn get_character(id: usize, data: &State<ViewerData>) -> Html<String> {
    let data = data.data.lock().expect("lock shared data");
    Html(visualize_character(&data, id))
}

#[get("/")]
fn get_town(data: &State<ViewerData>) -> Html<String> {
    let data = data.data.lock().expect("lock shared data");
    Html(visualize_town(&data))
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
        .mount("/building", routes![get_buildings, get_building])
        .mount("/character", routes![get_characters, get_character])
        .mount("/town", routes![get_town])
        .launch()
        .await
    {
        println!("Rocket didn't launch!");
        drop(e);
    };
}
