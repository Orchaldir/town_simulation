use crate::model::character::relation::family::RelativeType::*;
use crate::model::character::relation::RelationType::Relative;
use crate::model::character::{CharacterId, CharacterMgr};
use crate::usecase::character::add_relation;
use crate::usecase::character::relation::get::*;

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

    child
}
