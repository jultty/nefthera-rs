use crate::space::*;
use crate::lore::locations::regions::fontain::fontain;

pub fn onnehera() -> Zone {
    Zone {
        name: "Onnehera",
        is_settlement: true,
        region: fontain(),
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

