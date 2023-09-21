use crate::space::*;
use crate::lore::locations::regions::fontain::fontain;

pub fn oppos_outskirts() -> Zone {
    Zone {
        name: "Oppos Outskirts",
        is_settlement: false,
        region: fontain(),
        range: AreaRange {
            min_x: 0,
            max_x: 0,
            min_y: -100000,
            max_y: -80000,
            max_z: 0,
            min_z: 0,
        },
    }
}

