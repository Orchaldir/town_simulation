use crate::model::character::relation::family::RelativeType;
use crate::model::character::relation::family::RelativeType::*;
use crate::model::character::relation::RelationType::*;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::usecase::character::relation::get::*;
use crate::usecase::character::{add_relation, add_relations};
use std::collections::HashSet;

pub fn birth(manager: &mut CharacterMgr, father: CharacterId, mother: CharacterId) -> CharacterId {
    let parents = [father, mother].into();
    let child = manager.create();
    let siblings = get_shared_children(manager, father, mother);
    let grandparents = combine(&parents, |id| get_parents(manager, id));
    let piblings = combine(&parents, |id| get_siblings(manager, id));
    let cousins = combine(&piblings, |id| get_children(manager, id));

    add_relations(manager, child, &grandparents, Relative(GrandChild));
    add_relations(manager, child, &cousins, Relative(Cousin));
    add_relations(manager, child, &piblings, Relative(Nibling));
    add_relations(manager, child, &siblings, Relative(Sibling));
    add_relations(manager, child, &parents, Relative(Child));

    let mut relatives = HashSet::new();
    relatives.extend(parents.clone());
    relatives.extend(siblings.clone());
    relatives.extend(grandparents.clone());
    relatives.extend(piblings.clone());
    relatives.extend(cousins.clone());

    add_in_laws(manager, child, &grandparents, &relatives, GrandChild);
    add_in_laws(manager, child, &cousins, &relatives, Cousin);
    add_in_laws(manager, child, &piblings, &relatives, Nibling);
    add_in_laws(manager, child, &siblings, &relatives, Sibling);

    child
}

fn add_in_laws(
    manager: &mut CharacterMgr,
    character: CharacterId,
    targets: &HashSet<CharacterId>,
    relatives: &HashSet<CharacterId>,
    relative_type: RelativeType,
) {
    let relation_type = InLaw(relative_type);

    for target in targets {
        for spouse in get_spouses(manager, *target) {
            if !relatives.contains(&spouse) {
                add_relation(manager, character, spouse, relation_type);
            }
        }
    }
}
