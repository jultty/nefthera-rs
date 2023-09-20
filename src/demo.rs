use crate::character::*;
use crate::space::*;

// TODO Move these character definitions to module lore
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
            x: 65,
            y: 41,
            z: 0,
            area: Area {
                name: "Temple of Catherine",
                x: 1,
                y: 1,
                z: 1,
                map: Map {
                    name: "Onnehera",
                    x: 1,
                    y: 0,
                    z: 0,
                    world: World {
                        name: "Fiji",
                        x: 0,
                        y: 0,
                        z: 0,
                    },
                },
            },
        },
    }
}

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
            x: 15,
            y: 11,
            z: 0,
            area: Area {
                name: "Ibonhaun Laboratory",
                x: 1,
                y: 1,
                z: 1,
                map: Map {
                    name: "Oppos Outskirts",
                    x: 0,
                    y: 0,
                    z: 0,
                    world: World {
                        name: "Fiji",
                        x: 0,
                        y: 0,
                        z: 0,
                    },
                },
            },
        },
    }
}
