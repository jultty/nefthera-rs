use crate::base::character::*;
use crate::base::space::Position;
use crate::lore::locations::zones::oppos_outskirts::ibonhaun_lab;

pub fn ian() -> Character {
    Character {
        name: "Ian McHotter".into(),
        title: "The Dreamer".into(),
        hp: HP { current: 1, max: 1 },
        mp: MP { current: 1, max: 1 },
        dexterity: 40,
        strength: 10,
        intelligence: 1853,
        social: 15,
        perception: 1332,
        grit: 540,
        position: Position {
            area: ibonhaun_lab::instantiate(),
            x: 15,
            y: 11,
            z: 0,
        },
    }
}
