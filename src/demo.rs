use crate::character::*;
use crate::space::*;

// TODO Move character definitions to lore module

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
            area: onnehera_temple_of_catherine(),
            x: 65,
            y: 41,
            z: 0,
        },
    }
}

fn onnehera_temple_of_catherine() -> Area {
    Area {
        name: "Temple of Catherine",
        map: onnehera(),
        range: AreaRange {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            max_z: 0,
            min_z: 0,
        },
    }
}

fn onnehera() -> Map {
    Map {
        name: "Onnehera",
        world: fiji(),
        range: AreaRange {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            max_z: 0,
            min_z: 0,
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
            area: ibonhaun_lab(),
            x: 15,
            y: 11,
            z: 0,
        },
    }
}

fn ibonhaun_lab() -> Area {
    Area {
        name: "Ibonhaun Laboratory",
        map: oppos_outskirts(),
        range: AreaRange {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            max_z: 0,
            min_z: 0,
        },
    }
}

fn oppos_outskirts() -> Map {
    Map {
        name: "Oppos Outskirts",
        world: fiji(),
        range: AreaRange {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            max_z: 0,
            min_z: 0,
        },
    }
}

fn fiji() -> World {
    World {
        name: "Fiji",
        range: AreaRange {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            max_z: 0,
            min_z: 0,
        },
    }
}
