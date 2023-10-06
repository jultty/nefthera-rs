use crate::base::character::*;
use crate::base::space::entity::EntityMap;
use crate::lore::characters::ian::ian;

pub fn get_demo_character() -> Character {
    ian()
}

pub fn get_entity_map() -> EntityMap {
    EntityMap::new()
}
