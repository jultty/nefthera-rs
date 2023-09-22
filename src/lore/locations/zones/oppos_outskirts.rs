pub mod ibonhaun_lab;

use crate::base::space::*;
use crate::lore::locations::regions::fontain;

pub fn instantiate() -> Zone {
    Zone {
        name: "Oppos Outskirts",
        is_settlement: false,
        region: fontain::instantiate(),
        limits: SpaceLimits {
            min_x: -12000,
            max_x: -10000,
            min_y: -72000,
            max_y: -70000,
            max_z: 1000,
            min_z: 1000,
        },
    }
}
