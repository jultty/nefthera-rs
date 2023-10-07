use crate::base::character::*;
use crate::base::space::entity::{new_populated, EntityMap};
use crate::lore::characters::ian::ian;

pub fn get_demo_character() -> Character {
    ian()
}

pub fn get_entity_map() -> EntityMap {
    new_populated()
}
