use crate::base::space::Area;
use crate::base::space::AreaLimits;
use crate::lore::locations::zones::onnehera;

pub fn instantiate() -> Area {
    Area {
        name: "Temple of Catherine",
        zone: onnehera::instantiate(),
        limits: AreaLimits {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            max_z: 0,
            min_z: 0,
        },
    }
}
