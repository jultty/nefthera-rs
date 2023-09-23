use crate::base::character::*;
use crate::lore::characters::ian::ian;
use crate::lore::locations::passages::*;

pub fn get_demo_character() -> Character {
    ian()
}

pub fn get_passage_map() -> PassageMap {
    populate()
}
