pub mod ibonhaun_lab;

use crate::base::space::*;
use crate::lore::locations::regions::fontain;

pub fn instantiate() -> Zone {
    Zone {
        name: "Oppos Outskirts",
        is_settlement: false,
        region: fontain::instantiate(),
        limits: AreaLimits {
            min_x: 0,
            max_x: 0,
            min_y: -100000,
            max_y: -80000,
            max_z: 0,
            min_z: 0,
        },
    }
}
