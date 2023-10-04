use crate::base::description::Description;
use crate::base::space::limit::SpaceLimits;
use crate::base::space::passage::Passage;
use crate::base::space::units::{Area, Position};
use crate::lore::locations::zones::oppos_outskirts;

pub fn instantiate() -> Area {
    Area {
        name: "Ibonhaun Laboratory",
        key: "OPPOS-OUTSKIRTS_IBONHAUN-LABORATORY",
        zone: oppos_outskirts::instantiate(),
        description: Description::instantiate("A small cabin in the woods, found after following a barely visible path that strays from the main road to the larger Ibonhaun Academy, leading through the Oppos Woods to a clearing where the original laboratory of Alexander Ibonhaun still could be reached."),
        limits: SpaceLimits {
            min_x: -11000,
            max_x: -10000,
            min_y: -71000,
            max_y: -70000,
            max_z: 1000,
            min_z: -1000,
        },
    }
}

pub fn get_to_oppos_woods() -> Passage {
    Passage::initialize(
        "Passage to Oppos Woods",
        "IBONHAUN-LAB_TO_OPPOS-WOODS",
        true,
        Position {
            area: instantiate(),
            x: -10500,
            y: -70999,
            z: 0,
        },
        Position {
            area: oppos_outskirts::oppos_woods::instantiate(),
            x: -10500,
            y: -710001,
            z: 0,
        },
    )
}
