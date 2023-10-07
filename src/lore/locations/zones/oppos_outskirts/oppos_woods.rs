use crate::base::description::Description;
use crate::base::space::limit::SpaceLimits;
use crate::base::space::passage::Passage;
use crate::base::space::units::Area;
use crate::base::space::units::Position;
use crate::lore::locations::zones::oppos_outskirts;

pub fn instantiate() -> Area {
    Area {
        name: "Oppos Woods",
        key: "OPPOS-OUTSKIRTS_OPPOS-WOODS",
        zone: oppos_outskirts::instantiate(),
        description: Description::new(
            "The Oppos Woods were a mixture of expansive fields and areas covered by trees.",
        ),
        limits: SpaceLimits {
            min_x: -11000,
            max_x: -10000,
            min_y: -72000,
            max_y: -71000,
            max_z: 1000,
            min_z: -1000,
        },
    }
}

pub fn get_to_ibonhaun_lab() -> Passage {
    Passage::initialize(
        "Passage to the Ibonhaun Laboratory",
        "OPPOS-WOODS_TO_IBONHAUN-LAB",
        true,
        Position {
            area: instantiate(),
            x: -10500,
            y: -710001,
            z: 0,
        },
        Position {
            area: oppos_outskirts::ibonhaun_lab::instantiate(),
            x: -10500,
            y: -70999,
            z: 0,
        },
    )
}
