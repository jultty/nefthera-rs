pub mod alexander_ibonhaun;
pub mod bertha;
pub mod ian;

use crate::base::character::Character;

pub fn get_vector() -> Vec<Character> {
    vec![alexander_ibonhaun::new(), ian::ian()]
}
