use crate::visualize::building::{show_building_id_link, show_building_link};
use crate::visualize::html;
use town_simulation::model::building::BuildingMgr;
use town_simulation::model::character::relation::building::BuildingRelationType::{Builder, Owner};
use town_simulation::model::character::relation::building::{
    BuildingRelation, BuildingRelationType,
};
use town_simulation::model::character::relation::character::CharacterRelation;
use town_simulation::model::character::{Character, CharacterId, CharacterMgr};
use town_simulation::model::time::Date;
use town_simulation::usecase::building::occupancy::get_building_occupied_by;
use town_simulation::usecase::character::relation::get::{
    get_relation_to_in_laws, get_relation_to_relatives, get_spouses,
};
use town_simulation::SimulationData;

pub fn visualize_characters(data: &SimulationData) -> String {
    let manager = &data.character_manager;
    let total = manager.get_all().len();
    let alive = manager.get_all().iter().filter(|&c| c.is_alive()).count();
    let dead = total - alive;
    html(format!(
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
        show_character_list(manager.get_all(), data.date),
    ))
}

pub fn visualize_character(data: &SimulationData, id: usize) -> String {
    let manager = &data.character_manager;
    let character_id = CharacterId::new(id);

    if let Some(character) = manager.get(character_id) {
        html(format!(
            "
  <h1>{}</h1>
  <h2>General</h2>
  <p><b>Id:</b> {}</p>
  <p><b>Gender:</b> {:?}</p>
  <p><b>Birth Date:</b> {}</p>{}
  <p><b>Age:</b> {}</p>
  <h2>Characters</h2>{}{}{}
  <h2>Buildings</h2>{}{}{}
  <a href=\"/character\">Back</a>",
            character.name(),
            id,
            character.gender(),
            character.birth_date().get_year(),
            show_death(character),
            character.get_age(data.date),
            show_spouse(manager, character_id),
            show_relatives(manager, character_id),
            show_in_laws(manager, character_id),
            show_home(data, character_id),
            show_build_buildings(&data.building_manager, character.building_relations()),
            show_owned_buildings(&data.building_manager, character.building_relations()),
        ))
    } else {
        html(format!(
            "
  <h1>Unknown Character {}!</h1>
  <a href=\"/character\">Back</a>",
            id,
        ))
    }
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
        "   <li>{} (Age: {})</li>",
        show_character_link(character),
        character.get_age(date),
    )
}

pub fn show_character_id_link(manager: &CharacterMgr, id: CharacterId) -> String {
    show_character_link(manager.get(id).unwrap())
}

pub fn show_character_link(character: &Character) -> String {
    format!(
        "<a href=\"/character/{}\">{}</a>",
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

fn show_relations(
    manager: &CharacterMgr,
    mut relations: Vec<&CharacterRelation>,
    text: &str,
) -> String {
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

fn show_relation(manager: &CharacterMgr, relation: &CharacterRelation) -> String {
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

fn show_home(data: &SimulationData, id: CharacterId) -> String {
    if let Some(building_id) = get_building_occupied_by(&data.character_manager, id) {
        format!(
            "\n<p><b>Home:</b> {}</p>",
            show_building_id_link(&data.building_manager, building_id),
        )
    } else {
        "".to_string()
    }
}

fn show_build_buildings(manager: &BuildingMgr, relations: &[BuildingRelation]) -> String {
    show_building_relations(manager, relations, Builder)
}

fn show_owned_buildings(manager: &BuildingMgr, relations: &[BuildingRelation]) -> String {
    show_building_relations(manager, relations, Owner)
}

fn show_building_relations(
    manager: &BuildingMgr,
    relations: &[BuildingRelation],
    relation_type: BuildingRelationType,
) -> String {
    let vector: Vec<String> = relations
        .iter()
        .filter(|&relation| *relation.relation_type() == relation_type)
        .map(|relation| show_building_relation(manager, relation))
        .collect();

    if vector.is_empty() {
        "".to_string()
    } else {
        format!(
            "
  <p><b>{:?}:</b></p>
  <ul>
    {}
  </ul>
  ",
            relation_type,
            vector.join("\n")
        )
    }
}

fn show_building_relation(manager: &BuildingMgr, relation: &BuildingRelation) -> String {
    let building = manager.get(*relation.id()).unwrap();

    format!(
        "   <li>{:?} of {}</li>",
        relation.relation_type(),
        show_building_link(building),
    )
}
