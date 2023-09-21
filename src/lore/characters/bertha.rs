use crate::character::*;
use crate::space::Position;
use crate::lore::locations::areas::temple_of_catherine::temple_of_catherine;

pub fn bertha() -> Character {
    Character {
        name: "Bertha".into(),
        title: "The Neck Crusher".into(),
        hp: HP {
            current: 1000,
            max: 1000,
        },
        mp: MP {
            current: 200,
            max: 200,
        },
        dexterity: 36,
        strength: 232,
        intelligence: 20,
        social: 2,
        perception: 101,
        grit: 600,
        position: Position {
            area: temple_of_catherine(),
            x: 65,
            y: 41,
            z: 0,
        },
    }
}

