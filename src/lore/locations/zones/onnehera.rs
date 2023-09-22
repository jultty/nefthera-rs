pub mod temple_of_catherine;

use crate::base::space::*;
use crate::lore::locations::regions::fontain;

pub fn instantiate() -> Zone {
    Zone {
        name: "Onnehera",
        is_settlement: true,
        region: fontain::instantiate(),
        limits: SpaceLimits {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            max_z: 0,
            min_z: 0,
        },
    }
}
