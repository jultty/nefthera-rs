use crate::base::character::*;
use crate::base::space::passage::PassageMap;
use crate::lore::characters::ian::ian;
use crate::lore::locations::passages::populate;

pub fn get_demo_character() -> Character {
    ian()
}

pub fn get_passage_map() -> PassageMap {
    populate()
}
