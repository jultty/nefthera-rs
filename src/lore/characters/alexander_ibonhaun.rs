use crate::base::character::*;
use crate::base::space::units::Position;
use crate::lore::locations::zones::oppos_outskirts::ibonhaun_lab;

pub fn new() -> Character {
    Character {
        name: "Alexander Ibonhaun".into(),
        title: "The Brewer".into(),
        hp: HP {
            current: 600,
            max: 600,
        },
        mp: MP {
            current: 300,
            max: 400,
        },
        dexterity: 20,
        strength: 7,
        intelligence: 253,
        social: 30,
        perception: 102,
        grit: 220,
        position: Position {
            area: ibonhaun_lab::instantiate(),
            x: -10500,
            y: -70950,
            z: 0,
        },
    }
}
