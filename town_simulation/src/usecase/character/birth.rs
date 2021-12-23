use crate::model::character::relation::family::RelativeType;
use crate::model::character::relation::family::RelativeType::*;
use crate::model::character::relation::Relation;
use crate::model::character::relation::RelationType::*;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::usecase::character::add_relation;
use crate::usecase::character::relation::get::*;
use std::collections::HashSet;

pub fn birth(manager: &mut CharacterMgr, father: CharacterId, mother: CharacterId) -> CharacterId {
    let parents = [father, mother].into();
    let child = manager.create();
    let siblings = get_shared_children(manager, father, mother);
    let grandparents = combine(&parents, |id| get_parents(manager, id));
    let piblings = combine(&parents, |id| get_siblings(manager, id));
    let cousins = combine(&piblings, |id| get_children(manager, id));

    add_relation(manager, child, &grandparents, Relative(GrandChild));
    add_relation(manager, child, &cousins, Relative(Cousin));
    add_relation(manager, child, &piblings, Relative(Nibling));
    add_relation(manager, child, &siblings, Relative(Sibling));
    add_relation(manager, child, &parents, Relative(Child));

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

pub fn add_in_laws(
    manager: &mut CharacterMgr,
    character: CharacterId,
    targets: &HashSet<CharacterId>,
    relatives: &HashSet<CharacterId>,
    relative_type: RelativeType,
) {
    let relation = Relation::new(InLaw(relative_type), character);
    let other_type = InLaw(relative_type.reverse());

    for target in targets {
        for spouse in get_spouses(manager, *target) {
            if !relatives.contains(&spouse) {
                manager.get_mut(spouse).unwrap().relations.push(relation);

                let other_relation = Relation::new(other_type, *target);

                manager
                    .get_mut(spouse)
                    .unwrap()
                    .relations
                    .push(other_relation);
            }
        }
    }
}
