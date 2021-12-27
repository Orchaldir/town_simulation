use crate::generation::name::NameGenerator;
use crate::model::character::gender::Gender::Male;
use crate::model::character::name::CharacterName;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::usecase::character::relation::get::get_parents;

#[derive(Default, Debug)]
pub struct CharacterNameGenerator {
    male_names: NameGenerator,
    female_names: NameGenerator,
    family_names: NameGenerator,
}

impl CharacterNameGenerator {
    pub fn new(
        male_names: NameGenerator,
        female_names: NameGenerator,
        family_names: NameGenerator,
    ) -> Self {
        Self {
            male_names,
            female_names,
            family_names,
        }
    }

    pub fn load(setting: &str) -> Self {
        Self::new(
            NameGenerator::read(&format!("{}-male.csv", setting)),
            NameGenerator::read(&format!("{}-female.csv", setting)),
            NameGenerator::read(&format!("{}-family.csv", setting)),
        )
    }

    pub fn generate(&self, manager: &CharacterMgr, id: CharacterId) -> CharacterName {
        let first_name = self.generate_first(manager, id);
        let family_name = self.generate_family(manager, id);

        CharacterName::standard(first_name, family_name)
    }

    fn generate_first(&self, manager: &CharacterMgr, id: CharacterId) -> &str {
        let character = manager.get(id).unwrap();

        self.generate_name(
            if *character.gender() == Male {
                &self.male_names
            } else {
                &self.female_names
            },
            id,
        )
    }

    fn generate_family<'a>(&'a self, manager: &'a CharacterMgr, id: CharacterId) -> &'a str {
        get_parents(manager, id)
            .iter()
            .map(|parent_id| manager.get(*parent_id))
            .flatten()
            .map(|parent| parent.name().get_last())
            .flatten()
            .next()
            .unwrap_or_else(|| self.generate_name(&self.family_names, id))
    }

    fn generate_name<'a>(&'a self, generator: &'a NameGenerator, id: CharacterId) -> &'a str {
        generator.get(id.id() as u32 * 1000000)
    }
}
