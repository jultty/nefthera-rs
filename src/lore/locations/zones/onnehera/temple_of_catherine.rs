use crate::base::description::Description;
use crate::base::space::limit::SpaceLimits;
use crate::base::space::units::Area;
use crate::lore::locations::zones::onnehera;

pub fn instantiate() -> Area {
    Area {
        name: "Temple of Catherine",
        key: "ONNEHERA_TEMPLE-OF-CATHERINE",
        zone: onnehera::instantiate(),
        description: Description::instantiate("Not added yet"),
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
