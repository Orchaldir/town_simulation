use crate::visualize::html;
use town_simulation::model::character::relation::Relation;
use town_simulation::model::character::{Character, CharacterId, CharacterMgr};
use town_simulation::model::time::Date;
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
  <h2>Relations</h2>{}{}{}
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
