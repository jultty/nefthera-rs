use crate::base::space::limit::SpaceLimits;
use crate::base::space::passage::Passage;
use crate::base::space::units::Area;
use crate::base::space::units::Position;
use crate::lore::locations::zones::oppos_outskirts;

pub fn instantiate() -> Area {
    Area {
        name: "Ibonhaun Laboratory",
        key: "OPPOS-OUTSKIRTS_IBONHAUN-LABORATORY",
        zone: oppos_outskirts::instantiate(),
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
